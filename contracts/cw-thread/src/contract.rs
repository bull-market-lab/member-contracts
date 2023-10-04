use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    Uint64,
};

use thread::config::{Config, FeeShareConfig, ProtocolFeeConfig};
use thread::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{CONFIG, NEXT_THREAD_ID};
use crate::{execute, query, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin_addr: deps
            .api
            .addr_validate(&msg.admin_addr.unwrap_or(info.sender.to_string()))?,
        registration_admin_addr: deps.api.addr_validate(
            &msg.registration_admin_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        protocol_fee_collector_addr: deps.api.addr_validate(
            &msg.protocol_fee_collector_addr
                .unwrap_or(info.sender.to_string()),
        )?,
        // TODO: P1: use noble USDC?
        fee_denom: msg.fee_denom.unwrap_or("uluna".to_string()),
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

        protocol_fee_config: ProtocolFeeConfig {
            // Default to 10%
            // e.g. user pays 10 LUNA to buy 5 keys
            // Assume key issuer uses default_trading_fee_percentage_of_key which is 5%
            // And key issuer uses default_key_trading_fee_share_config which is 50% for key issuer and 50% for key holder
            // In total user pays 10.55 LUNA
            // 0.25 LUNA goes to key issuer, 0.25 LUNA gets splitted by all key holders proportionally
            // 0.05 (because 10% of 0.5 is 0.05) LUNA goes to protocol fee collector
            key_trading_fee_percentage: msg
                .protocol_fee_key_trading_fee_percentage
                .unwrap_or(Uint64::from(10_u64)),
            // Default to 10_000 uluna, i.e.e 0.01 luna
            start_new_thread_fixed_cost: msg
                .protocol_fee_start_new_thread_fixed_cost
                .unwrap_or(Uint128::from(10_000_u64)),
            // Default to 0%
            ask_in_thread_fee_percentage: msg
                .protocol_fee_ask_in_thread_fee_percentage
                .unwrap_or(Uint64::zero()),
            // Default to 0%
            reply_in_thread_fee_percentage: msg
                .protocol_fee_reply_in_thread_fee_percentage
                .unwrap_or(Uint64::zero()),
        },

        // By default, pay 5% of the total price of buying or selling amount of key to buy or sell
        default_trading_fee_percentage_of_key: msg
            .default_trading_fee_percentage_of_key
            .unwrap_or(Uint64::from(5_u64)),
        // By default, pay 5% of the price of a single key to ask
        default_ask_fee_percentage_of_key: msg
            .default_ask_fee_percentage_of_key
            .unwrap_or(Uint64::from(5_u64)),
        // By default, pay 1% of the price of a single key to reply
        default_reply_fee_percentage_of_key: msg
            .default_reply_fee_percentage_of_key
            .unwrap_or(Uint64::one()),

        default_key_trading_fee_share_config: FeeShareConfig {
            // Default to 50%
            key_issuer_fee_percentage: msg
                .default_key_trading_fee_key_holder_fee_percentage
                .unwrap_or(Uint64::from(50_u64)),
            // Default to 50%
            key_holder_fee_percentage: msg
                .default_key_trading_fee_key_issuer_fee_percentage
                .unwrap_or(Uint64::from(50_u64)),
        },
        default_thread_fee_share_config: FeeShareConfig {
            // Default to 50%
            key_issuer_fee_percentage: msg
                .default_thread_fee_key_holder_fee_percentage
                .unwrap_or(Uint64::from(50_u64)),
            // Default to 50%
            key_holder_fee_percentage: msg
                .default_thread_fee_key_issuer_fee_percentage
                .unwrap_or(Uint64::from(50_u64)),
        },
    };

    if config
        .default_key_trading_fee_share_config
        .key_holder_fee_percentage
        + config
            .default_key_trading_fee_share_config
            .key_issuer_fee_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::KeyTradingFeeSharePercentageMustBe100 {});
    }

    if config
        .default_thread_fee_share_config
        .key_holder_fee_percentage
        + config
            .default_thread_fee_share_config
            .key_issuer_fee_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::ThreadFeeSharePercentageMustBe100 {});
    }

    CONFIG.save(deps.storage, &config)?;

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
    let config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::UpdateConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::config::update_config(deps, info, data)
        }
        ExecuteMsg::Register() => {
            cw_utils::nonpayable(&info)?;
            execute::user::register(deps, info)
        }
        ExecuteMsg::LinkSocialMedia(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::link_social_media(deps, info, data, config)
        }
        ExecuteMsg::RegisterKey(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::register_key(deps, info, data, config)
        }
        ExecuteMsg::UpdateTradingFeePercentageOfKey(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_trading_fee_percentage_of_key(deps, info, data)
        }
        ExecuteMsg::UpdateAskFeePercentageOfKey(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_ask_fee_percentage_of_key(deps, info, data)
        }
        ExecuteMsg::UpdateReplyFeePercentageOfKey(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_reply_fee_percentage_of_key(deps, info, data)
        }
        ExecuteMsg::UpdateKeyTradingFeeShareConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_key_trading_fee_share_config(deps, info, data)
        }
        ExecuteMsg::UpdateThreadFeeShareConfig(data) => {
            cw_utils::nonpayable(&info)?;
            execute::user::update_thread_fee_share_config(deps, info, data)
        }
        ExecuteMsg::BuyKey(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::key::buy_key(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::SellKey(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::key::sell_key(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::StartNewThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::thread::start_new_thread(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::AskInThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::thread::ask_in_thread(deps, env, info, data, config, user_paid_amount)
        }
        ExecuteMsg::AnswerInThread(data) => {
            cw_utils::nonpayable(&info)?;
            execute::thread::answer_in_thread(deps, info, data, config)
        }
        ExecuteMsg::ReplyInThread(data) => {
            let user_paid_amount = cw_utils::must_pay(&info, config.fee_denom.as_str())?;
            execute::thread::reply_in_thread(deps, env, info, data, config, user_paid_amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig(_) => to_binary(&query::config::query_config(deps)?),
        QueryMsg::QueryUser(data) => to_binary(&query::user::query_user(deps, data)?),
        QueryMsg::QueryKeySupply(data) => to_binary(&query::key::query_key_supply(deps, data)?),
        QueryMsg::QueryKeyHolders(data) => {
            to_binary(&query::key_holder::query_key_holders(deps, data)?)
        }
        QueryMsg::QueryUserHoldings(data) => {
            to_binary(&query::user_holding::query_user_holdings(deps, data)?)
        }
        QueryMsg::QueryCostToBuyKey(data) => {
            to_binary(&query::key::query_cost_to_buy_key(deps, data)?)
        }
        QueryMsg::QueryCostToSellKey(data) => {
            to_binary(&query::key::query_cost_to_sell_key(deps, data)?)
        }
        QueryMsg::QueryCostToStartNewThread(data) => {
            to_binary(&query::thread::query_cost_to_start_new_thread(deps, data)?)
        }
        QueryMsg::QueryCostToAsk(data) => to_binary(&query::thread::query_cost_to_ask(deps, data)?),
        QueryMsg::QueryCostToReply(data) => {
            to_binary(&query::thread::query_cost_to_reply(deps, data)?)
        }
        QueryMsg::QueryIDsOfAllThreadsUserBelongTo(data) => to_binary(
            &query::thread::query_ids_of_all_threads_user_belong_to(deps, data)?,
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
