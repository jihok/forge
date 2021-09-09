use cosmwasm_std::to_binary;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{
    Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, InitResponse, Querier, StdResult,
    Storage, WasmMsg,
};
use cw20::MinterResponse;
use terraswap::hook::InitHook as Cw20InitHook;
use terraswap::token::InitMsg as Cw20InitMsg;

use crate::handler::core as Handler;
use crate::handler::query as QueryHandler;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::querier;
use crate::state;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let sender = env.message.sender;
    let raw_sender = deps.api.canonical_address(&sender)?;

    let mut config = state::Config {
        this: deps.api.canonical_address(&env.contract.address)?,
        owner: raw_sender,
        pylon_pool: deps.api.canonical_address(&msg.pylon_pool)?,
        stable_denom: String::default(),
        dp_token: CanonicalAddr::default(),
    };

    let pool_config = querier::config(deps, &config.pylon_pool)?;

    config.stable_denom = pool_config.stable_denom.clone();

    state::store(&mut deps.storage, &config)?;

    Ok(InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: msg.dp_code_id,
            send: vec![],
            label: None,
            msg: to_binary(&Cw20InitMsg {
                name: format!("Deposit Token - {}", msg.pool_name),
                symbol: "ForgeDP".to_string(),
                decimals: 6u8,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: env.contract.address.clone(),
                    cap: None,
                }),
                init_hook: Some(Cw20InitHook {
                    contract_addr: env.contract.address,
                    msg: to_binary(&HandleMsg::RegisterDPToken {})?,
                }),
            })?,
        })],
        log: vec![],
    });
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::RegisterDPToken {} => Handler::register_dp_token(deps, env),
        HandleMsg::Deposit {} => Handler::deposit(deps, env),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::DepositAmountOf { owner } => QueryHandler::deposit_amount(deps, owner), // TODO: should accept a RiskLevel enum
        QueryMsg::TotalDepositAmount {} => QueryHandler::total_deposit_amount(deps), // TODO: should accept a RiskLevel enum
        QueryMsg::Config {} => QueryHandler::config(deps),
    }
}
