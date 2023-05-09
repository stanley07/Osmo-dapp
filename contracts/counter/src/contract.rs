#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        todos: msg.todos,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("todos", format!("{:?}", msg.todos)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { todo } => execute::add(deps, info, todo),
        ExecuteMsg::Remove { todo } => execute::remove(deps, info, todo),
        ExecuteMsg::Update { todo } => execute::update(deps, info, todo),
        ExecuteMsg::Reset { todo } => execute::reset(deps, info),
    }
}

pub mod execute {
    use crate::msg::Todo;

    use super::*;

    pub fn add(deps: DepsMut, info: MessageInfo, todo:Todo) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner{
                return Err(ContractError::Unauthorized { })
            }
            state.todos.push(todo)
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "add"))
    }

    pub fn remove(deps: DepsMut, info: MessageInfo, todo:Todo) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner{
                return Err(ContractError::Unauthorized { })
            }
            state.todos.retain(|x| x.id != todo.id);
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "remove"))
    }

    pub fn update(deps: DepsMut, info: MessageInfo, todo:Todo) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner{
                return Err(ContractError::Unauthorized { })
            }
            state.todos.retain(|x| x.id != todo.id);
            state.todos.push(todo)
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "update"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.todos.clear();
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTodos {} => to_binary(&query::get_todos(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_todos(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { todos: state.todos })
    }
}

