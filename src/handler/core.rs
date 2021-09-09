use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{
    log, to_binary, Api, Coin, CosmosMsg, Env, Extern, HandleResponse, Querier, StdError,
    StdResult, Storage, WasmMsg,
};
use cw20::Cw20HandleMsg;
use moneymarket::querier::deduct_tax;

use crate::querier;
use crate::state;

pub fn deposit<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let config = state::read(&deps.storage)?;

    // check deposit
    let received: Uint256 = env
        .message
        .sent_funds
        .iter()
        .find(|c| c.denom == config.stable_denom)
        .map(|c| Uint256::from(c.amount))
        .unwrap_or_else(Uint256::zero);

    if received.is_zero() {
        return Err(StdError::generic_err(format!(
            "Pool: insufficient token amount {}",
            config.stable_denom,
        )));
    }
    if env.message.sent_funds.len() > 1 {
        return Err(StdError::generic_err(format!(
            "Pool: this contract only accepts {}",
            config.stable_denom,
        )));
    }

    let dp_mint_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: received.into(),
        },
    )?
    .amount;

    Ok(HandleResponse {
        messages: [
            querier::pool_deposit_msg(deps, &config.pool, &config.stable_denom, received.into())?,
            vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.human_address(&config.dp_token)?,
                msg: to_binary(&Cw20HandleMsg::Mint {
                    recipient: env.message.sender.clone(),
                    amount: dp_mint_amount.clone(),
                })?,
                send: vec![],
            })],
        ]
        .concat(),
        log: vec![
            log("action", "deposit"),
            log("sender", env.message.sender),
            log("amount", dp_mint_amount),
        ],
        data: None,
    })
}

pub fn register_dp_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut config = config::read(&deps.storage)?;
    if config.dp_token != CanonicalAddr::default() {
        return Err(StdError::unauthorized());
    }

    config.dp_token = deps.api.canonical_address(&env.message.sender)?;
    config::store(&mut deps.storage, &config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("dp_token", env.message.sender)],
        data: None,
    })
}

pub fn redeem<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    env: Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let config = config::read(&deps.storage)?;

    let epoch_state = querier::anchor::epoch_state(deps, &config.moneymarket)?;
    let market_redeem_amount = Uint256::from(amount).div(epoch_state.exchange_rate);
    let user_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: amount.into(),
        },
    )?;

    Ok(HandleResponse {
        messages: [
            vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.human_address(&config.dp_token)?,
                msg: to_binary(&Cw20HandleMsg::Burn { amount })?,
                send: vec![],
            })],
            querier::anchor::redeem_stable_msg(
                deps,
                &config.moneymarket,
                &config.atoken,
                market_redeem_amount.into(),
            )?,
            vec![CosmosMsg::Bank(BankMsg::Send {
                from_address: env.contract.address,
                to_address: sender,
                amount: vec![user_redeem_amount.clone()],
            })],
        ]
        .concat(),
        log: vec![
            log("action", "redeem"),
            log("sender", env.message.sender),
            log("amount", user_redeem_amount.amount),
        ],
        data: None,
    })
}
