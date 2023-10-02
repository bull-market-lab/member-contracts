use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, Uint64,
};

use thread::{
    config::Config,
    msg::{AnswerMsg, AskMsg, QueryMsg, QuerySimulateAskMsg, SimulateAskResponse},
    qa_thread::QAThread,
    qa_thread_msg::{Answer, QAThreadMsg, Question},
};

use crate::{
    state::{
        ALL_QA_THREADS, ALL_QA_THREADS_MSGS, ALL_QA_THREADS_USERS_BELONG_TO, ALL_USERS_HOLDINGS,
        NEXT_QA_THREAD_ID, NEXT_QA_THREAD_MSG_ID,
    },
    ContractError,
};

pub fn ask(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    data: AskMsg,
    config: Config,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    if ALL_USERS_HOLDINGS
        .load(deps.storage, (&info.sender, &data.ask_to_addr))
        .unwrap_or(Uint128::zero())
        == Uint128::zero()
    {
        return Err(ContractError::UserMustHoldKeyToAsk {});
    }

    let title_len = data
        .qa_thread_title
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .count() as u64;
    if title_len > config.max_qa_thread_title_length.u64() {
        return Err(ContractError::QAThreadTitleTooLong {
            max: config.max_qa_thread_title_length.u64(),
            actual: title_len,
        });
    }

    if data.content.chars().count() > config.max_qa_thread_msg_length.u64() as usize {
        return Err(ContractError::QAThreadMsgContentTooLong {
            max: config.max_qa_thread_msg_length.u64(),
            actual: data.content.chars().count() as u64,
        });
    }

    let simulate_ask_response: SimulateAskResponse = deps.querier.query_wasm_smart(
        env.contract.address,
        &QueryMsg::QuerySimulateAsk(QuerySimulateAskMsg {
            ask_to_addr: data.ask_to_addr.clone(),
            content_len: Uint128::from(data.content.chars().count() as u128),
        }),
    )?;

    if simulate_ask_response.total_needed_from_user > user_paid_amount {
        return Err(ContractError::InsufficientFundsToPayDuringAsk {
            needed: simulate_ask_response.total_needed_from_user,
            available: user_paid_amount,
        });
    }

    let (qa_thread_id, qa_thread_msg_id) = if data.start_new_qa_thread.unwrap_or(false) {
        (NEXT_QA_THREAD_ID.load(deps.storage)?, Uint64::one())
    } else {
        (
            data.qa_thread_id.unwrap(),
            NEXT_QA_THREAD_MSG_ID.load(deps.storage, data.qa_thread_id.unwrap().u64())?,
        )
    };

    if data.start_new_qa_thread.unwrap_or(false) {
        ALL_QA_THREADS.update(
            deps.storage,
            qa_thread_id.u64(),
            |qa_thread| match qa_thread {
                None => {
                    let qa_thread = QAThread {
                        id: qa_thread_id,
                        title: data.qa_thread_title.unwrap(),
                        description: data.qa_thread_description.unwrap(),
                        ask_to_addr: data.ask_to_addr.to_string(),
                    };
                    Ok(qa_thread)
                }
                Some(_) => Err(ContractError::QAThreadAlreadyExist {}),
            },
        )?;
        // Bump next_available_qa_thread_id
        NEXT_QA_THREAD_ID.save(deps.storage, &(qa_thread_id + Uint64::one()))?;
        // Set next_available_qa_thread_msg_id to 2 as we just have 1 question message now
        NEXT_QA_THREAD_MSG_ID.save(deps.storage, qa_thread_id.u64(), &Uint64::from(2 as u64))?;
    } else {
        // Bump next_available_qa_thread_msg_id
        NEXT_QA_THREAD_MSG_ID.update(
            deps.storage,
            qa_thread_id.u64(),
            |next_available_qa_thread_msg_id| match next_available_qa_thread_msg_id {
                None => Err(ContractError::QAThreadNotExist {}),
                Some(next_available_qa_thread_msg_id) => {
                    Ok(next_available_qa_thread_msg_id + Uint64::one())
                }
            },
        )?;
    }

    // Add to asker's list of QA threads they belong to
    ALL_QA_THREADS_USERS_BELONG_TO.save(
        deps.storage,
        (&info.sender, qa_thread_id.u64()),
        &qa_thread_id,
    )?;
    // Add to answerer's list of QA threads they belong to
    ALL_QA_THREADS_USERS_BELONG_TO.save(
        deps.storage,
        (&data.ask_to_addr, qa_thread_id.u64()),
        &qa_thread_id,
    )?;

    ALL_QA_THREADS_MSGS.update(
        deps.storage,
        (qa_thread_id.u64(), qa_thread_msg_id.u64()),
        |qa_thread_msg| match qa_thread_msg {
            None => {
                let new_question = QAThreadMsg::Question(Question {
                    id: qa_thread_msg_id,
                    qa_thread_id,
                    ask_by_addr: info.sender,
                    content: data.content,
                });
                Ok(new_question)
            }
            Some(_) => Err(ContractError::QAThreadMsgAlreadyExist {}),
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
                amount: simulate_ask_response.key_issuer_fee,
            }],
        }),
        // Send protocol fee to fee collector
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.protocol_fee_collector_addr.to_string(),
            amount: vec![Coin {
                denom: config.fee_denom.clone(),
                amount: simulate_ask_response.protocol_fee,
            }],
        }),
    ];

    Ok(Response::new().add_messages(msgs_vec))
}

pub fn answer(
    deps: DepsMut,
    info: MessageInfo,
    data: AnswerMsg,
    config: Config,
) -> Result<Response, ContractError> {
    let qa_thread = ALL_QA_THREADS.load(deps.storage, data.qa_thread_id.u64())?;

    if qa_thread.ask_to_addr != info.sender {
        return Err(ContractError::OnlyKeyIssuerCanAnswer {});
    }

    let qa_thread_msg_id = NEXT_QA_THREAD_MSG_ID.load(deps.storage, data.qa_thread_id.u64())?;

    // Bump next_available_qa_thread_msg_id
    NEXT_QA_THREAD_MSG_ID.update(
        deps.storage,
        qa_thread.id.u64(),
        |next_available_qa_thread_msg_id| match next_available_qa_thread_msg_id {
            None => Err(ContractError::QAThreadNotExist {}),
            Some(next_available_qa_thread_msg_id) => {
                Ok(next_available_qa_thread_msg_id + Uint64::one())
            }
        },
    )?;

    if data.content.chars().count() > config.max_qa_thread_msg_length.u64() as usize {
        return Err(ContractError::QAThreadMsgContentTooLong {
            max: config.max_qa_thread_msg_length.u64(),
            actual: data.content.chars().count() as u64,
        });
    }

    ALL_QA_THREADS_MSGS.update(
        deps.storage,
        (data.qa_thread_id.u64(), data.question_id.u64()),
        |qa_thread_msg| match qa_thread_msg {
            None => {
                let new_answer = QAThreadMsg::Answer(Answer {
                    id: qa_thread_msg_id,
                    qa_thread_id: data.qa_thread_id,
                    answer_by_addr: info.sender,
                    content: data.content,
                    reply_to_question_id: data.question_id,
                });
                Ok(new_answer)
            }
            Some(_) => Err(ContractError::QAThreadMsgNotExist {}),
        },
    )?;

    Ok(Response::new())
}
