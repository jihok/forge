use crate::state;
use cosmwasm_std::{
    to_binary, Api, Binary, CanonicalAddr, Extern, HumanAddr, Querier, QueryRequest, StdResult,
    Storage, Uint128, WasmQuery,
};
use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositAmountResponse {
    pub amount: Uint128,
}

pub fn balance_of<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    token: &CanonicalAddr,
    owner: HumanAddr,
) -> StdResult<Uint128> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: deps.api.human_address(token)?,
        msg: to_binary(&Cw20QueryMsg::Balance { address: owner })?,
    }))?;

    Ok(balance.balance)
}

pub fn deposit_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    owner: HumanAddr,
) -> StdResult<Binary> {
    let config: state::Config = state::read(&deps.storage)?;

    to_binary(&DepositAmountResponse {
        amount: balance_of(deps, &config.dp_token, owner)?,
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalDepositAmountResponse {
    pub amount: Uint128,
}

pub fn total_supply<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    token: &CanonicalAddr,
) -> StdResult<Uint128> {
    let token_info: TokenInfoResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(token)?,
            msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
        }))?;

    Ok(token_info.total_supply)
}

pub fn total_deposit_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Binary> {
    let config: state::Config = state::read(&deps.storage)?;

    to_binary(&TotalDepositAmountResponse {
        amount: total_supply(deps, &config.dp_token)?,
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub pylon_pool: HumanAddr,
    pub stable_denom: String,
    pub dp_token: HumanAddr,
}

pub fn config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let config: state::Config = state::read(&deps.storage)?;

    to_binary(&ConfigResponse {
        pylon_pool: deps.api.human_address(&config.pylon_pool)?,
        stable_denom: config.stable_denom,
        dp_token: deps.api.human_address(&config.dp_token)?,
    })
}
