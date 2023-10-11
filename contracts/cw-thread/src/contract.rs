use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    Uint64,
};

use thread::config::Config;
use thread::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{CONFIG, NEXT_THREAD_ID};
use crate::util::fee_share::assert_config_fee_share_sum_to_100;
use crate::util::membership::query_membership_contract_config;
use crate::{execute, query, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        membership_contract_addr: deps.api.addr_validate(&msg.membership_contract_addr)?,
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        enabled: false,
        protocol_fee_collector_addr: deps.api.addr_validate(
            &msg.protocol_fee_collector_addr
                .unwrap_or(info.sender.to_string()),
        )?,

        // TODO: P0: benchmark how much gas it costs to store a 100, 250, 500, 1000 characters string
        // If there's a huge difference then introduce new param that will charge more as the length of the string increases
        max_thread_title_length: msg.max_thread_title_length.unwrap_or(Uint64::from(100_u64)),
        max_thread_description_length: msg
            .max_thread_description_length
            .unwrap_or(Uint64::from(500_u64)),
        max_thread_msg_length: msg.max_thread_msg_length.unwrap_or(Uint64::from(500_u64)),
        max_thread_label_length: msg.max_thread_msg_length.unwrap_or(Uint64::from(10_u64)),
        max_number_of_thread_labels: msg
            .max_number_of_thread_labels
            .unwrap_or(Uint64::from(5_u64)),

        // Default to 10_000 uluna, i.e 0.01 luna
        protocol_fee_start_new_thread_fixed_cost: msg
            .protocol_fee_start_new_thread_fixed_cost
            .unwrap_or(Uint128::from(10_000_u64)),
        // Default to 0%
        protocol_fee_ask_in_thread_fee_percentage: msg
            .protocol_fee_ask_in_thread_fee_percentage
            .unwrap_or(Uint64::zero()),
        // Default to 0%
        protocol_fee_reply_in_thread_fee_percentage: msg
            .protocol_fee_reply_in_thread_fee_percentage
            .unwrap_or(Uint64::zero()),

        // By default, pay 5% of the price of a single membership to ask
        default_ask_fee_percentage_of_membership: msg
            .default_ask_fee_percentage_of_membership
            .unwrap_or(Uint64::from(5_u64)),
        // By default, pay 1% of the price of a single membership to thread creator when someone ask in thread
        default_ask_fee_to_thread_creator_percentage_of_membership: msg
            .default_ask_fee_to_thread_creator_percentage_of_membership
            .unwrap_or(Uint64::one()),
        // By default, pay 1% of the price of a single membership to reply
        default_reply_fee_percentage_of_membership: msg
            .default_reply_fee_percentage_of_membership
            .unwrap_or(Uint64::one()),
        // By default, pay 1% of the price of a single membership to thread creator when someone reply in thread
        default_reply_fee_to_thread_creator_percentage_of_membership: msg
            .default_reply_fee_to_thread_creator_percentage_of_membership
            .unwrap_or(Uint64::one()),

        default_share_to_issuer_percentage: msg
            .default_share_to_issuer_percentage
            .unwrap_or(Uint64::from(50_u64)),

        default_share_to_all_members_percentage: msg
            .default_share_to_all_members_percentage
            .unwrap_or(Uint64::from(50_u64)),
    };

    CONFIG.save(deps.storage, &config)?;
    assert_config_fee_share_sum_to_100(deps.as_ref())?;

    NEXT_THREAD_ID.save(deps.storage, &Uint64::one())?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let deps_ref = deps.as_ref();
    let config = CONFIG.load(deps_ref.storage)?;
    let membership_contract_config =
        query_membership_contract_config(deps_ref, config.clone().membership_contract_addr.clone());
    let fee_denom = membership_contract_config.fee_denom.as_str();
    let distribution_contract_addr = membership_contract_config
        .distribution_contract_addr
        .unwrap();
    let membership_contract_addr = config.clone().membership_contract_addr;

    match msg {
        ExecuteMsg::Enable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::enable(deps, info)
        }
        ExecuteMsg::Disable(_) => {
            cw_utils::nonpayable(&info)?;
            execute::config::disable(deps, info)
        }
        ExecuteMsg::UpdateConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::update_config(deps, info, data)
        }
        ExecuteMsg::UpdateUserConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user_config::update_user_config(deps, info, data, membership_contract_addr)
        }
        ExecuteMsg::StartNewThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, fee_denom)?;
            execute::thread::start_new_thread(
                deps,
                env,
                info,
                data,
                config,
                fee_denom.to_string(),
                user_paid_amount,
            )
        }
        ExecuteMsg::AskInThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, fee_denom)?;
            execute::thread::ask_in_thread(
                deps,
                env,
                info,
                data,
                config,
                fee_denom.to_string(),
                distribution_contract_addr,
                user_paid_amount,
            )
        }
        ExecuteMsg::AnswerInThread(data) => {
            cw_utils::nonpayable(&info)?;
            execute::thread::answer_in_thread(deps, info, data, config)
        }
        ExecuteMsg::ReplyInThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, fee_denom)?;
            execute::thread::reply_in_thread(
                deps,
                env,
                info,
                data,
                config,
                fee_denom.to_string(),
                distribution_contract_addr,
                user_paid_amount,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUserConfig(data) => {
            to_binary(&query::user_config::query_user_config(deps, data)?)
        }
        QueryMsg::QueryCostToStartNewThread(_) => to_binary(
            &query::thread::query_cost_to_start_new_thread(config)?,
        ),
        QueryMsg::QueryCostToAskInThread(data) => to_binary(
            &query::thread::query_cost_to_ask_in_thread(deps, data, config)?,
        ),
        QueryMsg::QueryCostToReplyInThread(data) => to_binary(
            &query::thread::query_cost_to_reply_in_thread(deps, data, config)?,
        ),
        QueryMsg::QueryIDsOfAllThreadsUserParticipated(data) => to_binary(
            &query::thread::query_ids_of_all_threads_user_participated(deps, data)?,
        ),
        QueryMsg::QueryIDsOfAllThreadsUserCreated(data) => to_binary(
            &query::thread::query_ids_of_all_threads_user_created(deps, data)?,
        ),
        QueryMsg::QueryIDsOfAllThreadMsgsInThread(data) => to_binary(
            &query::thread::query_ids_of_all_thread_msgs_in_thread(deps, data)?,
        ),
        QueryMsg::QueryThreadsByIDs(data) => {
            to_binary(&query::thread::query_threads_by_ids(deps, data)?)
        }
        QueryMsg::QueryThreadMsgsByIDs(data) => {
            to_binary(&query::thread::query_thread_msgs_by_ids(deps, data)?)
        }
    }
}
