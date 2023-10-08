use crate::state::CONFIG;
use crate::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint64};
use thread::msg::UpdateConfigMsg;

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    data: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin_addr {
        return Err(ContractError::OnlyAdminCanUpdateConfig {});
    }

    config.admin_addr = match data.admin_addr {
        None => config.admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.registration_admin_addr = match data.registration_admin_addr {
        None => config.registration_admin_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.protocol_fee_collector_addr = match data.protocol_fee_collector_addr {
        None => config.protocol_fee_collector_addr,
        Some(data) => deps.api.addr_validate(data.as_str())?,
    };

    config.fee_denom = data.fee_denom.unwrap_or(config.fee_denom);

    config.max_thread_title_length = data
        .max_thread_title_length
        .unwrap_or(config.max_thread_title_length);

    config.max_thread_description_length = data
        .max_thread_description_length
        .unwrap_or(config.max_thread_description_length);

    config.max_thread_msg_length = data
        .max_thread_msg_length
        .unwrap_or(config.max_thread_msg_length);

    config.max_thread_label_length = data
        .max_thread_label_length
        .unwrap_or(config.max_thread_label_length);

    config.max_number_of_thread_labels = data
        .max_number_of_thread_labels
        .unwrap_or(config.max_number_of_thread_labels);

    config.protocol_fee_config.key_trading_fee_percentage = data
        .protocol_fee_key_trading_fee_percentage
        .unwrap_or(config.protocol_fee_config.key_trading_fee_percentage);

    config.protocol_fee_config.start_new_thread_fixed_cost = data
        .protocol_fee_start_new_thread_fixed_cost
        .unwrap_or(config.protocol_fee_config.start_new_thread_fixed_cost);

    config.protocol_fee_config.ask_in_thread_fee_percentage = data
        .protocol_fee_ask_in_thread_fee_percentage
        .unwrap_or(config.protocol_fee_config.ask_in_thread_fee_percentage);

    config.protocol_fee_config.reply_in_thread_fee_percentage = data
        .protocol_fee_reply_in_thread_fee_percentage
        .unwrap_or(config.protocol_fee_config.reply_in_thread_fee_percentage);

    config.default_trading_fee_percentage_of_key = data
        .default_trading_fee_percentage_of_key
        .unwrap_or(config.default_trading_fee_percentage_of_key);

    config.default_ask_fee_percentage_of_key = data
        .default_ask_fee_percentage_of_key
        .unwrap_or(config.default_ask_fee_percentage_of_key);

    config.default_ask_fee_to_thread_creator_percentage_of_key = data
        .default_ask_fee_to_thread_creator_percentage_of_key
        .unwrap_or(config.default_ask_fee_to_thread_creator_percentage_of_key);

    config.default_reply_fee_percentage_of_key = data
        .default_reply_fee_percentage_of_key
        .unwrap_or(config.default_reply_fee_percentage_of_key);

    config
        .default_key_trading_fee_share_config
        .key_issuer_fee_percentage = data
        .default_key_trading_fee_key_issuer_fee_percentage
        .unwrap_or(
            config
                .default_key_trading_fee_share_config
                .key_issuer_fee_percentage,
        );

    config
        .default_key_trading_fee_share_config
        .key_holder_fee_percentage = data
        .default_key_trading_fee_key_holder_fee_percentage
        .unwrap_or(
            config
                .default_key_trading_fee_share_config
                .key_holder_fee_percentage,
        );

    if config
        .default_key_trading_fee_share_config
        .key_holder_fee_percentage
        + config
            .default_key_trading_fee_share_config
            .key_issuer_fee_percentage
        != Uint64::from(100_u64)
    {
        return Err(ContractError::MembershipTradingFeeSharePercentageMustBe100 {});
    }

    config
        .default_thread_fee_share_config
        .key_issuer_fee_percentage = data.default_thread_fee_key_issuer_fee_percentage.unwrap_or(
        config
            .default_thread_fee_share_config
            .key_issuer_fee_percentage,
    );

    config
        .default_thread_fee_share_config
        .key_holder_fee_percentage = data.default_thread_fee_key_holder_fee_percentage.unwrap_or(
        config
            .default_thread_fee_share_config
            .key_holder_fee_percentage,
    );

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

    Ok(Response::new().add_attribute("action", "update_config"))
}
