use cosmwasm_std::Coin;
use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{
    Storage, Api, Querier, Extern, CanonicalAddr, Uint128, StdResult, CosmosMsg,
    WasmMsg, to_binary, WasmQuery, QueryRequest,
};
use moneymarket::querier::deduct_tax;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    DepositAmountOf {
        owner: HumanAddr,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: CanonicalAddr,
    pub moneymarket: CanonicalAddr,
    pub atoken: CanonicalAddr,
    pub stable_denom: String,
    pub dp_token: CanonicalAddr,
}

pub fn config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pool: &CanonicalAddr,
) -> StdResult<ConfigResponse> {
    let pool_config: ConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(pool)?,
            msg: to_binary(&QueryMsg::Config {})?,
        }))?;

    Ok(pool_config)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Deposit {},
}

pub fn pool_deposit_msg<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pool: &CanonicalAddr,
    denom: &str,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.human_address(pool)?,
        msg: to_binary(&HandleMsg::Deposit {})?,
        send: vec![deduct_tax(
            deps,
            Coin {
                denom: denom.to_string(),
                amount,
            },
        )?],
    })])
}
