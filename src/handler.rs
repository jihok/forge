
use moneymarket::querier::deduct_tax;
use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{
    CosmosMsg, Coin, StdError, HandleResponse, StdResult, Env, Extern, Querier, Api,
    Storage, WasmMsg, log, to_binary,
};
use cw20::Cw20HandleMsg;

use crate::state;
use crate::querier;

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
            querier::pool_deposit_msg(
                deps,
                &config.pool,
                &config.stable_denom,
                received.into(),
            )?,
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
