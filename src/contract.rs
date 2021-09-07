use cosmwasm_std::{
    Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, InitResponse, Querier, StdResult,
    Storage,
};

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
        pool: deps.api.canonical_address(&msg.pool)?,
        stable_denom: String::default(),
        dp_token: CanonicalAddr::default(),
    };

    let pool_config = querier::config(deps, &config.pool)?;

    config.stable_denom = pool_config.stable_denom.clone();

    state::store(&mut deps.storage, &config)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Deposit { pool_type } => Handler::deposit(deps, env, pool_type),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::DepositAmountOf { owner } => QueryHandler::deposit_amount(deps, owner),
        QueryMsg::TotalDepositAmount {} => QueryHandler::total_deposit_amount(deps),
        QueryMsg::Config {} => QueryHandler::config(deps),
    }
}
