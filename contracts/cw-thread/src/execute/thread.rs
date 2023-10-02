use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Uint64,
};

use thread::{
    config::Config,
    msg::{
        AnswerInThreadMsg, AskInThreadMsg, CostToAskResponse, CostToReplyResponse,
        CostToStartNewThreadResponse, QueryCostToAskMsg, QueryCostToReplyMsg,
        QueryCostToStartNewThreadMsg, QueryMsg, ReplyInThreadMsg, StartNewThreadMsg,
    },
    thread::Thread,
    thread_msg::{ThreadAnswerMsg, ThreadMsg, ThreadQuestionMsg, ThreadReplyMsg},
};

use crate::{
    state::{
        ALL_THREADS, ALL_THREADS_MSGS, ALL_THREADS_UNANSWERED_QUESTION_MSGS,
        ALL_THREADS_USERS_BELONG_TO, ALL_THREADS_USERS_CREATED, ALL_USERS_HOLDINGS, NEXT_THREAD_ID,
        NEXT_THREAD_MSG_ID, USERS,
    },
    ContractError,
};

pub fn start_new_thread(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: StartNewThreadMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_ref = &info.sender;
    let user = match USERS.load(deps.storage, sender_ref) {
        Ok(user) => user,
        Err(_) => return Err(ContractError::UserNotExist {}),
    };

    // TODO: P1: allow user to start thread without having issued key, maybe a thread only itself can interact with
    if !user.issued_key {
        return Err(ContractError::UserMustHaveIssuedKeyToStartNewThread {});
    }

    let title_len = data.title.clone().chars().count() as u64;
    if title_len > config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadTitleTooLong {
            max: config.max_thread_title_length.u64(),
            actual: title_len,
        });
    }

    let description_len = data.description.clone().chars().count() as u64;
    if description_len > config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadDescriptionTooLong {
            max: config.max_thread_msg_length.u64(),
            actual: description_len,
        });
    }

    let cost_to_start_new_thread_response: CostToStartNewThreadResponse =
        deps.querier.query_wasm_smart(
            env.contract.address,
            &QueryMsg::QueryCostToStartNewThread(QueryCostToStartNewThreadMsg {
                description_len: Uint64::from(data.description.chars().count() as u64),
            }),
        )?;

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
                creator_addr: info.sender.clone(),
            };
            Ok(thread)
        }
        Some(_) => Err(ContractError::ThreadAlreadyExist {}),
    })?;
    ALL_THREADS_USERS_CREATED.update(deps.storage, (sender_ref, thread_id.u64()), |thread| {
        match thread {
            None => Ok(true),
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        }
    })?;
    ALL_THREADS_USERS_BELONG_TO.update(deps.storage, (sender_ref, thread_id.u64()), |thread| {
        match thread {
            None => Ok(true),
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        }
    })?;
    // Bump next_available_thread_id
    NEXT_THREAD_ID.save(deps.storage, &(thread_id + Uint64::one()))?;

    let msgs_vec = vec![
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_start_new_thread_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn ask_in_thread(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: AskInThreadMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_ref = &info.sender;
    let ask_to_addr_ref = &deps.api.addr_validate(data.ask_to_addr.as_str()).unwrap();

    if ALL_USERS_HOLDINGS
        .load(deps.storage, (sender_ref, ask_to_addr_ref))
        .unwrap_or(Uint128::zero())
        == Uint128::zero()
    {
        return Err(ContractError::UserMustHoldKeyToAsk {});
    }

    let title_len = data
        .thread_title
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .count() as u64;
    if title_len > config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadTitleTooLong {
            max: config.max_thread_title_length.u64(),
            actual: title_len,
        });
    }

    let content_len = data.content.chars().count() as u64;
    if content_len > config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: config.max_thread_msg_length.u64(),
            actual: content_len,
        });
    }

    let cost_to_ask_response: CostToAskResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QueryCostToAsk(QueryCostToAskMsg {
            ask_to_addr: data.ask_to_addr.clone(),
            content_len: Uint64::from(content_len),
        }),
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

    if data.start_new_thread.unwrap_or(false) {
        ALL_THREADS.update(deps.storage, thread_id.u64(), |thread| match thread {
            None => {
                let thread = Thread {
                    id: thread_id,
                    title: data.thread_title.unwrap(),
                    description: data.thread_description.unwrap(),
                    labels: data.thread_labels.unwrap_or(vec![]),
                    creator_addr: info.sender.clone(),
                };
                Ok(thread)
            }
            Some(_) => Err(ContractError::ThreadAlreadyExist {}),
        })?;
        ALL_THREADS_USERS_CREATED.update(
            deps.storage,
            (sender_ref, thread_id.u64()),
            |thread| match thread {
                None => Ok(true),
                Some(_) => Err(ContractError::ThreadAlreadyExist {}),
            },
        )?;
        // Bump next_available_thread_id
        NEXT_THREAD_ID.save(deps.storage, &(thread_id + Uint64::one()))?;
        // Set next_available_thread_msg_id to 2 as we just have 1 question message now
        NEXT_THREAD_MSG_ID.save(deps.storage, thread_id.u64(), &Uint64::from(2 as u64))?;
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
    }

    // Add to asker's list of threads they belong to
    ALL_THREADS_USERS_BELONG_TO.save(deps.storage, (&info.sender, thread_id.u64()), &true)?;

    // Add to unanswered question list
    ALL_THREADS_UNANSWERED_QUESTION_MSGS.save(
        deps.storage,
        (thread_id.u64(), thread_msg_id.u64()),
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
                    creator_addr: info.sender,
                    content: data.content,
                    asked_to_addr: ask_to_addr_ref.to_owned(),
                });
                Ok(new_question)
            }
            Some(_) => Err(ContractError::ThreadMsgAlreadyExist {}),
        },
    )?;

    // TODO: do not send key issuer fee to key issuer until question is answered
    // TODO: decide if we want to hold payout to key holders as well, i think we should, give it more pressure to answer
    // We can do those fancy trick later, as now if i ask a question and not get answer, i won't ask again

    // TODO: P0 feature: distribute key holder fee to all key holders
    // This would likely to be async that use warp because there could be a lot of key holders
    // If we do it here it might run out of gas

    let msgs_vec = vec![
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: data.ask_to_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_ask_response.key_issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_ask_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn answer_in_thread(
    deps: DepsMut,
    info: MessageInfo,
    data: AnswerInThreadMsg,
    config: Config,
) -> Result<Response, ContractError> {
    let question =
        ALL_THREADS_MSGS.load(deps.storage, (data.thread_id.u64(), data.question_id.u64()))?;

    let question = match question {
        ThreadMsg::ThreadAnswerMsg(_) => return Err(ContractError::ThreadMsgIsNotQuestion {}),
        ThreadMsg::ThreadQuestionMsg(question) => question,
        ThreadMsg::ThreadReplyMsg(_) => return Err(ContractError::ThreadMsgIsNotQuestion {}),
    };

    if question.asked_to_addr != info.sender {
        return Err(ContractError::CannotAnswerOthersQuestion {});
    }

    let thread_msg_id = NEXT_THREAD_MSG_ID.load(deps.storage, data.thread_id.u64())?;

    // Bump next_available_thread_msg_id
    NEXT_THREAD_MSG_ID.update(
        deps.storage,
        question.id.u64(),
        |next_available_thread_msg_id| match next_available_thread_msg_id {
            None => Err(ContractError::ThreadNotExist {}),
            Some(next_available_thread_msg_id) => Ok(next_available_thread_msg_id + Uint64::one()),
        },
    )?;

    if data.content.chars().count() > config.max_thread_msg_length.u64() as usize {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: config.max_thread_msg_length.u64(),
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
                    creator_addr: info.sender.clone(),
                    content: data.content,
                    answered_to_question_msg_id: data.question_id,
                });
                Ok(new_answer)
            }
            Some(_) => Err(ContractError::ThreadMsgNotExist {}),
        },
    )?;

    // Add to answerer's list of threads they belong to
    ALL_THREADS_USERS_BELONG_TO.save(deps.storage, (&info.sender, data.thread_id.u64()), &true)?;

    // Delete from unanswered question list
    ALL_THREADS_UNANSWERED_QUESTION_MSGS
        .remove(deps.storage, (data.thread_id.u64(), data.question_id.u64()));

    Ok(Response::new())
}

pub fn reply_in_thread(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: ReplyInThreadMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_ref = &info.sender;
    let reply_to_addr_ref = &match data.reply_to_thread_msg_id {
        Some(reply_to_thread_msg_id) => {
            let reply = ALL_THREADS_MSGS.load(
                deps.storage,
                (data.thread_id.u64(), reply_to_thread_msg_id.u64()),
            )?;
            match reply {
                ThreadMsg::ThreadAnswerMsg(answer) => answer.creator_addr,
                ThreadMsg::ThreadQuestionMsg(question) => question.creator_addr,
                ThreadMsg::ThreadReplyMsg(reply) => reply.creator_addr,
            }
        }
        None => {
            let thread = ALL_THREADS.load(deps.storage, data.thread_id.u64())?;
            thread.creator_addr
        }
    };

    if ALL_USERS_HOLDINGS
        .load(deps.storage, (sender_ref, reply_to_addr_ref))
        .unwrap_or(Uint128::zero())
        == Uint128::zero()
    {
        return Err(ContractError::UserMustHoldKeyToReply {});
    }

    let title_len = data.content.chars().count() as u64;
    if title_len > config.max_thread_title_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: config.max_thread_msg_length.u64(),
            actual: title_len,
        });
    }

    let content_len = data.content.chars().count() as u64;
    if content_len > config.max_thread_msg_length.u64() {
        return Err(ContractError::ThreadMsgContentTooLong {
            max: config.max_thread_msg_length.u64(),
            actual: content_len,
        });
    }
    let cost_to_reply_response: CostToReplyResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QueryCostToReply(QueryCostToReplyMsg {
            reply_to_addr: reply_to_addr_ref.to_string(),
            content_len: Uint64::from(content_len as u64),
        }),
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

    // Add to asker's list of threads they belong to
    ALL_THREADS_USERS_BELONG_TO.save(deps.storage, (sender_ref, data.thread_id.u64()), &true)?;

    ALL_THREADS_MSGS.update(
        deps.storage,
        (data.thread_id.u64(), thread_msg_id.u64()),
        |thread_msg| match thread_msg {
            None => {
                let new_question = ThreadMsg::ThreadReplyMsg(ThreadReplyMsg {
                    id: thread_msg_id,
                    content: data.content,
                    creator_addr: info.sender,
                    reply_to_thread_msg_id: data.reply_to_thread_msg_id,
                    thread_id: data.thread_id,
                });
                Ok(new_question)
            }
            Some(_) => Err(ContractError::ThreadMsgAlreadyExist {}),
        },
    )?;

    // TODO: do not send key issuer fee to key issuer until question is answered
    // TODO: decide if we want to hold payout to key holders as well, i think we should, give it more pressure to answer
    // We can do those fancy trick later, as now if i ask a question and not get answer, i won't ask again

    // TODO: P0 feature: distribute key holder fee to all key holders
    // This would likely to be async that use warp because there could be a lot of key holders
    // If we do it here it might run out of gas

    let msgs_vec = vec![
        // Send key issuer fee to key issuer
        CosmosMsg::Bank(BankMsg::Send {
            to_address: reply_to_addr_ref.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_reply_response.key_issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: cost_to_reply_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}
