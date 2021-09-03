use cosmwasm_std::CanonicalAddr;
use cosmwasm_std::{
    Api, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg};
use crate::state;
use crate::querier;
use crate::handler::{deposit};


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
        HandleMsg::Deposit {} => deposit(deps, env),
    }
}
