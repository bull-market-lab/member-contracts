use cosmwasm_std::{
    to_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, DepsMut, MessageInfo, Response, Uint128,
    Uint64, WasmMsg,
};

use distribution::msg::{DistributeMsg, ExecuteMsg};
use thread::{
    config::Config,
    msg::{
        AnswerInThreadMsg, AskInThreadMsg, CostToAskInThreadResponse, CostToReplyInThreadResponse,
        CostToStartNewThreadResponse, QueryCostToAskInThreadMsg, QueryCostToReplyInThreadMsg,
        ReplyInThreadMsg, StartNewThreadMsg,
    },
    thread::{Thread, ThreadAnswerMsg, ThreadMsg, ThreadQuestionMsg, ThreadReplyMsg},
};

use crate::{
    query::thread::{
        query_cost_to_ask_in_thread, query_cost_to_reply_in_thread, query_cost_to_start_new_thread,
    },
    state::{
        ALL_THREADS, ALL_THREADS_MSGS, ALL_THREADS_MSGS_COUNT, ALL_USERS_CREATED_THREADS,
        ALL_USERS_PARTICIPATED_THREADS, ALL_USERS_THREAD_STATS, ALL_USERS_UNANSWERED_QUESTIONS,
        NEXT_THREAD_ID, NEXT_THREAD_MSG_ID,
    },
    util::member::{
        query_is_user_a_member_and_membership_amount, query_membership_supply, query_user_by_addr,
        query_user_by_id,
    },
    ContractError,
};

pub fn start_new_thread(
    deps: DepsMut,
    info: MessageInfo,
    data: StartNewThreadMsg,
    config: Config,
    fee_denom: String,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let config_copy = config.clone();
    let thread_config = config.thread_config;
    let membership_contract_addr = config.membership_contract_addr;

    let thread_creator = query_user_by_addr(deps.as_ref(), membership_contract_addr, info.sender);
    let thread_creator_user_id = thread_creator.id.u64();

    // TODO: P1: allow user to start thread without having issued membership, maybe a thread only itself can interact with
    if thread_creator.membership_issued_by_me.is_none() {
        return Err(ContractError::UserMustHaveIssuedMembershipToStartNewThread {});
    }

    let title_len = data.title.clone().chars().count() as u64;
    if title_len > thread_config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadTitleTooLong {
            max: thread_config.max_thread_title_length.u64(),
            actual: title_len,
        });
    }

    let description_len = data.description.clone().chars().count() as u64;
    if description_len > thread_config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadDescriptionTooLong {
            max: thread_config.max_thread_msg_length.u64(),
            actual: description_len,
        });
    }

    let cost_to_start_new_thread_response: CostToStartNewThreadResponse =
        query_cost_to_start_new_thread(config_copy)?;

    if cost_to_start_new_thread_response.protocol_fee > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringAsk {
            needed: cost_to_start_new_thread_response.protocol_fee,
            available: user_paid_amount,
        });
    }

    let thread_id = NEXT_THREAD_ID.load(deps.storage)?;

    ALL_THREADS.update(deps.storage, thread_id.u64(), |thread| match thread {
        None => {
            let thread = Thread {
                id: thread_id,
                title: data.title,
                description: data.description,
                labels: data.labels,
                creator_user_id: Uint64::from(thread_creator_user_id),
                updatable: false,
                deletable: false,
            };
            Ok(thread)
        }
        Some(_) => Err(ContractError::ThreadAlreadyExist {}),
    })?;

    ALL_USERS_CREATED_THREADS.update(
        deps.storage,
        (thread_creator_user_id, thread_id.u64()),
        |thread| match thread {
            None => Ok(true),
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        },
    )?;
    ALL_USERS_PARTICIPATED_THREADS.update(
        deps.storage,
        (thread_creator_user_id, thread_id.u64()),
        |thread| match thread {
            None => Ok(true),
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        },
    )?;

    if ALL_USERS_THREAD_STATS.has(deps.storage, thread_creator_user_id) {
        ALL_USERS_THREAD_STATS.update(deps.storage, thread_creator_user_id, |thread_stats| {
            match thread_stats {
                None => Err(ContractError::UserNotExist {}),
                Some((created, participated)) => {
                    Ok((created + Uint128::one(), participated + Uint128::one()))
                }
            }
        })?;
    } else {
        ALL_USERS_THREAD_STATS.save(
            deps.storage,
            thread_creator_user_id,
            &(Uint128::one(), Uint128::one()),
        )?;
    }

    // Bump next_available_thread_id
    NEXT_THREAD_ID.save(deps.storage, &(thread_id + Uint64::one()))?;

    let msgs_vec = vec![
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: fee_denom,
                amount: cost_to_start_new_thread_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn ask_in_thread(
    deps: DepsMut,
    info: MessageInfo,
    data: AskInThreadMsg,
    config: Config,
    fee_denom: String,
    distribution_contract_addr: Addr,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let config_copy = config.clone();
    let thread_config = config.thread_config;
    let membership_contract_addr = config.membership_contract_addr;

    let asker = query_user_by_addr(deps.as_ref(), membership_contract_addr.clone(), info.sender);
    let asker_user_id = asker.id.u64();

    if asker.membership_issued_by_me.is_none() {
        return Err(ContractError::UserMustHaveIssuedMembershipToAsk {});
    }

    let (thread_creator_user_id, thread_creator) = if data.start_new_thread.unwrap_or(false) {
        (asker_user_id, asker)
    } else {
        let thread = ALL_THREADS.load(deps.storage, data.thread_id.unwrap().u64())?;
        let thread_creator = query_user_by_id(
            deps.as_ref(),
            membership_contract_addr.clone(),
            thread.creator_user_id.u64(),
        );
        (thread_creator.id.u64(), thread_creator)
    };

    let ask_to_user = query_user_by_id(
        deps.as_ref(),
        membership_contract_addr.clone(),
        data.ask_to_user_id.u64(),
    );
    let ask_to_user_id = ask_to_user.id.u64();

    if !query_is_user_a_member_and_membership_amount(
        deps.as_ref(),
        membership_contract_addr.clone(),
        ask_to_user_id,
        asker_user_id,
    )
    .0
    {
        return Err(ContractError::UserMustHoldAskToUserMembershipToAsk {});
    }

    if thread_creator_user_id != asker_user_id
        && !query_is_user_a_member_and_membership_amount(
            deps.as_ref(),
            membership_contract_addr.clone(),
            thread_creator_user_id,
            asker_user_id,
        )
        .0
    {
        return Err(ContractError::UserMustHoldThreadCreatorMembershipToAskInItsThread {});
    }

    let title_len = data
        .thread_title
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .count() as u64;
    if title_len > thread_config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadTitleTooLong {
            max: thread_config.max_thread_title_length.u64(),
            actual: title_len,
        });
    }

    let content_len = data.content.chars().count() as u64;
    if content_len > thread_config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: thread_config.max_thread_msg_length.u64(),
            actual: content_len,
        });
    }

    let cost_to_ask_response: CostToAskInThreadResponse = query_cost_to_ask_in_thread(
        deps.as_ref(),
        QueryCostToAskInThreadMsg {
            asker_user_id: Uint64::from(asker_user_id),
            ask_to_user_id: Uint64::from(ask_to_user_id),
            thread_creator_user_id: Uint64::from(thread_creator_user_id),
            content_len: Uint64::from(content_len),
        },
        config_copy,
    )?;

    if cost_to_ask_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringAsk {
            needed: cost_to_ask_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let (thread_id, thread_msg_id) = if data.start_new_thread.unwrap_or(false) {
        (NEXT_THREAD_ID.load(deps.storage)?, Uint64::one())
    } else {
        (
            data.thread_id.unwrap(),
            NEXT_THREAD_MSG_ID.load(deps.storage, data.thread_id.unwrap().u64())?,
        )
    };

    // Whether starting a new thread
    if data.start_new_thread.unwrap_or(false) {
        ALL_THREADS.update(deps.storage, thread_id.u64(), |thread| match thread {
            None => {
                let thread = Thread {
                    id: thread_id,
                    title: data.thread_title.unwrap(),
                    description: data.thread_description.unwrap(),
                    labels: data.thread_labels.unwrap_or(vec![]),
                    creator_user_id: Uint64::from(asker_user_id),
                    updatable: false,
                    deletable: false,
                };
                Ok(thread)
            }
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        })?;
        ALL_USERS_CREATED_THREADS.update(
            deps.storage,
            (asker_user_id, thread_id.u64()),
            |thread| match thread {
                None => Ok(true),
                Some(_) => Err(ContractError::ThreadAlreadyExist {}),
            },
        )?;
        ALL_USERS_PARTICIPATED_THREADS.update(
            deps.storage,
            (asker_user_id, thread_id.u64()),
            |thread| match thread {
                None => Ok(true),
                Some(_) => Err(ContractError::ThreadAlreadyExist {}),
            },
        )?;
        // Bump next_available_thread_id
        NEXT_THREAD_ID.save(deps.storage, &(thread_id + Uint64::one()))?;
        // Set next_available_thread_msg_id to 2 as we just have 1 question message now
        NEXT_THREAD_MSG_ID.save(deps.storage, thread_id.u64(), &Uint64::from(2_u64))?;
    } else {
        // Bump next_available_thread_msg_id
        NEXT_THREAD_MSG_ID.update(
            deps.storage,
            thread_id.u64(),
            |next_available_thread_msg_id| match next_available_thread_msg_id {
                None => Err(ContractError::ThreadNotExist {}),
                Some(next_available_thread_msg_id) => {
                    Ok(next_available_thread_msg_id + Uint64::one())
                }
            },
        )?;
        ALL_USERS_PARTICIPATED_THREADS.update(
            deps.storage,
            (asker_user_id, thread_id.u64()),
            |thread| match thread {
                None => Ok(true),
                Some(_) => Err(ContractError::ThreadAlreadyExist {}),
            },
        )?;
        ALL_USERS_THREAD_STATS.update(deps.storage, thread_creator_user_id, |thread_stats| {
            match thread_stats {
                None => Err(ContractError::UserNotExist {}),
                Some((created, participated)) => {
                    Ok((created + Uint128::one(), participated + Uint128::one()))
                }
            }
        })?;
    }

    if ALL_USERS_THREAD_STATS.has(deps.storage, ask_to_user_id) {
        ALL_USERS_THREAD_STATS.update(deps.storage, asker_user_id, |thread_stats| {
            match thread_stats {
                None => Err(ContractError::UserNotExist {}),
                Some((created, participated)) => {
                    Ok((created + Uint128::one(), participated + Uint128::one()))
                }
            }
        })?;
    } else {
        ALL_USERS_THREAD_STATS.save(
            deps.storage,
            asker_user_id,
            &(Uint128::one(), Uint128::one()),
        )?;
    }

    // Add to ask to's list of threads they belong to
    // ALL_THREADS_USERS_BELONG_TO.save(deps.storage, (&info.sender, thread_id.u64()), &true)?;
    ALL_USERS_PARTICIPATED_THREADS.update(
        deps.storage,
        (ask_to_user_id, thread_id.u64()),
        |thread| match thread {
            None => Ok(false),
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        },
    )?;

    // Add to unanswered question list
    ALL_USERS_UNANSWERED_QUESTIONS.save(
        deps.storage,
        (ask_to_user_id, thread_id.u64(), thread_msg_id.u64()),
        &true,
    )?;

    ALL_THREADS_MSGS.update(
        deps.storage,
        (thread_id.u64(), thread_msg_id.u64()),
        |thread_msg| match thread_msg {
            None => {
                let new_question = ThreadMsg::ThreadQuestionMsg(ThreadQuestionMsg {
                    id: thread_msg_id,
                    thread_id,
                    creator_user_id: Uint64::from(asker_user_id),
                    content: data.content,
                    asked_to_user_id: Uint64::from(ask_to_user_id),
                });
                Ok(new_question)
            }
            Some(_) => Err(ContractError::ThreadMsgAlreadyExist {}),
        },
    )?;
    if ALL_THREADS_MSGS_COUNT.has(deps.storage, thread_id.u64()) {
        ALL_THREADS_MSGS_COUNT.update(deps.storage, thread_id.u64(), |count| match count {
            None => Err(ContractError::ThreadNotExist {}),
            Some(count) => Ok(count + Uint128::one()),
        })?;
    } else {
        ALL_THREADS_MSGS_COUNT.save(deps.storage, thread_id.u64(), &Uint128::one())?;
    }

    let ask_to_membership_supply = query_membership_supply(
        deps.as_ref(),
        membership_contract_addr.clone(),
        ask_to_user_id,
    );
    let thread_creator_membership_supply = query_membership_supply(
        deps.as_ref(),
        membership_contract_addr,
        thread_creator_user_id,
    );

    // TODO: P1: do not send membership issuer fee to membership issuer until question is answered
    // TODO: P1: decide if we want to hold payout to membership holders as well, i think we should, give it more pressure to answer
    // We can do those fancy trick later, as now if i ask a question and not get answer, i won't ask again

    let mut msgs_vec = vec![];
    msgs_vec.push(
        // Send all member fee to distribution contract
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: distribution_contract_addr.to_string(),
            msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                membership_issuer_user_id: Uint64::from(ask_to_user_id),
                index_increment: Decimal::from_ratio(
                    cost_to_ask_response.ask_to_membership_all_members_fee,
                    ask_to_membership_supply,
                ),
            }))?,
            funds: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_ask_response.ask_to_membership_all_members_fee,
            }],
        }),
    );
    msgs_vec.push(
        // Send membership issuer fee to membership issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: ask_to_user.addr.to_string(),
            amount: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_ask_response.ask_to_membership_issuer_fee,
            }],
        }),
    );
    msgs_vec.push(
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: fee_denom.clone(),
                amount: cost_to_ask_response.protocol_fee,
            }],
        }),
    );

    // Send asker's question fee to thread creator if thread creator is not the asker
    if cost_to_ask_response.thread_creator_membership_all_members_fee > Uint128::zero() {
        msgs_vec.push(
            // Send all member fee to distribution contract
            // So it can send membership holder fee to thread creator's membership's holders
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: distribution_contract_addr.to_string(),
                msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                    membership_issuer_user_id: Uint64::from(thread_creator_user_id),
                    index_increment: Decimal::from_ratio(
                        cost_to_ask_response.thread_creator_membership_all_members_fee,
                        thread_creator_membership_supply,
                    ),
                }))?,
                funds: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_ask_response.thread_creator_membership_all_members_fee,
                }],
            }),
        );
    }
    if cost_to_ask_response.thread_creator_membership_issuer_fee > Uint128::zero() {
        msgs_vec.push(
            // Send membership issuer fee to thread creator
            CosmosMsg::Bank(BankMsg::Send {
                to_address: thread_creator.addr.to_string(),
                amount: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_ask_response.thread_creator_membership_issuer_fee,
                }],
            }),
        );
    }

    // TODO: P0: send tip to answerer

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn answer_in_thread(
    deps: DepsMut,
    info: MessageInfo,
    data: AnswerInThreadMsg,
    config: Config,
) -> Result<Response, ContractError> {
    let thread_config = config.thread_config;
    let membership_contract_addr = config.membership_contract_addr;

    let thread_id = data.thread_id.u64();
    let question_id = data.question_id.u64();

    let answerer = query_user_by_addr(deps.as_ref(), membership_contract_addr, info.sender);
    let answerer_user_id = answerer.id.u64();

    if answerer.membership_issued_by_me.is_none() {
        return Err(ContractError::UserMustHaveIssuedMembershipToAnswer {});
    }

    let question = ALL_THREADS_MSGS.load(deps.storage, (thread_id, question_id))?;

    let question = match question {
        ThreadMsg::ThreadAnswerMsg(_) => {
            return Err(ContractError::CannotAnswerNonQuestionThreadMsg {})
        }
        ThreadMsg::ThreadQuestionMsg(question) => question,
        ThreadMsg::ThreadReplyMsg(_) => {
            return Err(ContractError::CannotAnswerNonQuestionThreadMsg {})
        }
    };

    if question.asked_to_user_id.u64() != answerer_user_id {
        return Err(ContractError::CannotAnswerOthersQuestion {});
    }

    let thread_msg_id = NEXT_THREAD_MSG_ID.load(deps.storage, thread_id)?;

    // Bump next_available_thread_msg_id
    NEXT_THREAD_MSG_ID.update(
        deps.storage,
        question.id.u64(),
        |next_available_thread_msg_id| match next_available_thread_msg_id {
            None => Err(ContractError::ThreadNotExist {}),
            Some(next_available_thread_msg_id) => Ok(next_available_thread_msg_id + Uint64::one()),
        },
    )?;

    if data.content.chars().count() > thread_config.max_thread_msg_length.u64() as usize {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: thread_config.max_thread_msg_length.u64(),
            actual: data.content.chars().count() as u64,
        });
    }

    ALL_THREADS_MSGS.update(
        deps.storage,
        (data.thread_id.u64(), data.question_id.u64()),
        |thread_msg| match thread_msg {
            None => {
                let new_answer = ThreadMsg::ThreadAnswerMsg(ThreadAnswerMsg {
                    id: thread_msg_id,
                    thread_id: data.thread_id,
                    creator_user_id: Uint64::from(answerer_user_id),
                    content: data.content,
                    answered_to_question_msg_id: data.question_id,
                });
                Ok(new_answer)
            }
            Some(_) => Err(ContractError::ThreadMsgNotExist {}),
        },
    )?;

    if ALL_USERS_THREAD_STATS.has(deps.storage, answerer_user_id) {
        ALL_USERS_THREAD_STATS.update(deps.storage, answerer_user_id, |thread_stats| {
            match thread_stats {
                None => Err(ContractError::UserNotExist {}),
                Some((created, participated)) => {
                    Ok((created + Uint128::one(), participated + Uint128::one()))
                }
            }
        })?;
    } else {
        ALL_USERS_THREAD_STATS.save(
            deps.storage,
            answerer_user_id,
            &(Uint128::one(), Uint128::one()),
        )?;
    }

    // Add to answerer's list of threads they belong to
    ALL_USERS_PARTICIPATED_THREADS.save(
        deps.storage,
        (answerer_user_id, data.thread_id.u64()),
        &false,
    )?;

    // Delete from unanswered question list
    ALL_USERS_UNANSWERED_QUESTIONS.remove(
        deps.storage,
        (
            answerer_user_id,
            data.thread_id.u64(),
            data.question_id.u64(),
        ),
    );

    ALL_THREADS_MSGS_COUNT.update(deps.storage, thread_id, |count| match count {
        None => Err(ContractError::ThreadNotExist {}),
        Some(count) => Ok(count + Uint128::one()),
    })?;

    Ok(Response::new())
}

pub fn reply_in_thread(
    deps: DepsMut,
    info: MessageInfo,
    data: ReplyInThreadMsg,
    config: Config,
    fee_denom: String,
    distribution_contract_addr: Addr,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let config_copy = config.clone();
    let thread_config = config.thread_config;
    let membership_contract_addr = config.membership_contract_addr;

    let thread_id = data.thread_id.u64();

    let replier = query_user_by_addr(deps.as_ref(), membership_contract_addr.clone(), info.sender);
    let replier_user_id = replier.id.u64();

    if replier.membership_issued_by_me.is_none() {
        return Err(ContractError::UserMustHaveIssuedMembershipToReply {});
    }

    let thread = ALL_THREADS.load(deps.storage, thread_id)?;
    let thread_creator = query_user_by_id(
        deps.as_ref(),
        membership_contract_addr.clone(),
        thread.creator_user_id.u64(),
    );
    let thread_creator_user_id = thread_creator.id.u64();

    let (_, reply_to_user, reply_to_user_id) = if data.reply_to_thread_msg_id.is_some() {
        let reply_to_thread_msg = ALL_THREADS_MSGS.load(
            deps.storage,
            (
                data.thread_id.u64(),
                data.reply_to_thread_msg_id.unwrap().u64(),
            ),
        )?;
        let reply_to_user_id = match reply_to_thread_msg.clone() {
            ThreadMsg::ThreadAnswerMsg(answer) => answer.creator_user_id,
            ThreadMsg::ThreadQuestionMsg(question) => question.creator_user_id,
            ThreadMsg::ThreadReplyMsg(reply) => reply.creator_user_id,
        };
        let reply_to_user = query_user_by_id(
            deps.as_ref(),
            membership_contract_addr.clone(),
            reply_to_user_id.u64(),
        );
        (
            Some(reply_to_thread_msg),
            Some(reply_to_user),
            Some(reply_to_user_id.u64()),
        )
    } else {
        (None, None, None)
    };

    if !query_is_user_a_member_and_membership_amount(
        deps.as_ref(),
        membership_contract_addr.clone(),
        thread_creator_user_id,
        replier_user_id,
    )
    .0
    {
        return Err(ContractError::UserMustHoldThreadCreatorMembershipToReply {});
    }

    if reply_to_user_id.is_some()
        && !query_is_user_a_member_and_membership_amount(
            deps.as_ref(),
            membership_contract_addr.clone(),
            reply_to_user_id.unwrap(),
            replier_user_id,
        )
        .0
    {
        return Err(ContractError::UserMustHoldThreadReplyToUserMembershipToReply {});
    }

    let title_len = data.content.chars().count() as u64;
    if title_len > thread_config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: thread_config.max_thread_msg_length.u64(),
            actual: title_len,
        });
    }

    let content_len = data.content.chars().count() as u64;
    if content_len > thread_config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: thread_config.max_thread_msg_length.u64(),
            actual: content_len,
        });
    }
    let cost_to_reply_response: CostToReplyInThreadResponse = query_cost_to_reply_in_thread(
        deps.as_ref(),
        QueryCostToReplyInThreadMsg {
            replier_user_id: Uint64::from(replier_user_id),
            reply_to_user_id: Uint64::from(if reply_to_user_id.is_some() {
                reply_to_user_id.unwrap()
            } else {
                thread_creator_user_id
            }),
            thread_creator_user_id: Uint64::from(thread_creator_user_id),
            content_len: Uint64::from(content_len),
        },
        config_copy,
    )?;

    if cost_to_reply_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringAsk {
            needed: cost_to_reply_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let thread_msg_id = NEXT_THREAD_MSG_ID.load(deps.storage, data.thread_id.u64())?;

    // Bump next_available_thread_msg_id
    NEXT_THREAD_MSG_ID.update(
        deps.storage,
        data.thread_id.u64(),
        |next_available_thread_msg_id| match next_available_thread_msg_id {
            None => Err(ContractError::ThreadNotExist {}),
            Some(next_available_thread_msg_id) => Ok(next_available_thread_msg_id + Uint64::one()),
        },
    )?;

    ALL_THREADS_MSGS.update(
        deps.storage,
        (data.thread_id.u64(), thread_msg_id.u64()),
        |thread_msg| match thread_msg {
            None => {
                let new_question = ThreadMsg::ThreadReplyMsg(ThreadReplyMsg {
                    id: thread_msg_id,
                    content: data.content,
                    creator_user_id: Uint64::from(replier_user_id),
                    reply_to_thread_msg_id: data.reply_to_thread_msg_id,
                    thread_id: data.thread_id,
                });
                Ok(new_question)
            }
            Some(_) => Err(ContractError::ThreadMsgAlreadyExist {}),
        },
    )?;

    if ALL_USERS_THREAD_STATS.has(deps.storage, replier_user_id) {
        ALL_USERS_THREAD_STATS.update(deps.storage, replier_user_id, |thread_stats| {
            match thread_stats {
                None => Err(ContractError::UserNotExist {}),
                Some((created, participated)) => {
                    Ok((created + Uint128::one(), participated + Uint128::one()))
                }
            }
        })?;
    } else {
        ALL_USERS_THREAD_STATS.save(
            deps.storage,
            replier_user_id,
            &(Uint128::one(), Uint128::one()),
        )?;
    }

    // Add to replier's list of threads they belong to
    ALL_USERS_PARTICIPATED_THREADS.save(
        deps.storage,
        (replier_user_id, data.thread_id.u64()),
        &false,
    )?;

    ALL_THREADS_MSGS_COUNT.update(deps.storage, thread_id, |count| match count {
        None => Err(ContractError::ThreadNotExist {}),
        Some(count) => Ok(count + Uint128::one()),
    })?;

    let (reply_to_membership_supply, thread_creator_membership_supply) =
        if reply_to_user_id.is_some() {
            (
                query_membership_supply(
                    deps.as_ref(),
                    membership_contract_addr.clone(),
                    reply_to_user_id.unwrap(),
                ),
                query_membership_supply(
                    deps.as_ref(),
                    membership_contract_addr,
                    thread_creator_user_id,
                ),
            )
        } else {
            let thread_creator_membership_supply = query_membership_supply(
                deps.as_ref(),
                membership_contract_addr,
                thread_creator_user_id,
            );
            (
                thread_creator_membership_supply,
                thread_creator_membership_supply,
            )
        };

    let mut msgs_vec = vec![];
    if data.reply_to_thread_msg_id.is_some() {
        msgs_vec.push(
            // Send all member fee to distribution contract
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: distribution_contract_addr.to_string(),
                msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                    membership_issuer_user_id: Uint64::from(reply_to_user_id.unwrap()),
                    index_increment: Decimal::from_ratio(
                        cost_to_reply_response.reply_to_membership_all_members_fee,
                        reply_to_membership_supply,
                    ),
                }))?,
                funds: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_reply_response.reply_to_membership_all_members_fee,
                }],
            }),
        );
        msgs_vec.push(
            // Send membership issuer fee to membership issuer
            CosmosMsg::Bank(BankMsg::Send {
                to_address: reply_to_user.unwrap().addr.to_string(),
                amount: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_reply_response.reply_to_membership_issuer_fee,
                }],
            }),
        );
        msgs_vec.push(
            // Send protocol fee to fee collector
            CosmosMsg::Bank(BankMsg::Send {
                to_address: config.protocol_fee_collector_addr.to_string(),
                amount: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_reply_response.protocol_fee,
                }],
            }),
        );
    }

    // Send asker's question fee to thread creator if thread creator is not the asker
    if cost_to_reply_response.thread_creator_membership_all_members_fee > Uint128::zero() {
        msgs_vec.push(
            // Send all member fee to distribution contract
            // So it can send membership holder fee to thread creator's membership's holders
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: distribution_contract_addr.to_string(),
                msg: to_binary(&ExecuteMsg::Distribute(DistributeMsg {
                    membership_issuer_user_id: Uint64::from(thread_creator_user_id),
                    index_increment: Decimal::from_ratio(
                        cost_to_reply_response.thread_creator_membership_all_members_fee,
                        thread_creator_membership_supply,
                    ),
                }))?,
                funds: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_reply_response.thread_creator_membership_all_members_fee,
                }],
            }),
        );
    }
    if cost_to_reply_response.thread_creator_membership_issuer_fee > Uint128::zero() {
        msgs_vec.push(
            // Send membership issuer fee to thread creator
            CosmosMsg::Bank(BankMsg::Send {
                to_address: thread_creator.addr.to_string(),
                amount: vec![Coin {
                    denom: fee_denom.clone(),
                    amount: cost_to_reply_response.thread_creator_membership_issuer_fee,
                }],
            }),
        );
    }

    Ok(Response::new().add_messages(msgs_vec))
}
