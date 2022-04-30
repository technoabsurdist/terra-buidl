// main contract for contract terra project
use cosmwasm_std::entry_point; 
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult}; 
use cw2::set_contract_version; 

use crate::error::ContractError; 
use crate::msg::{CountResponse, InstantiateMsg, QueryMsg}; 
use crate::state::{State, STATE}; 


#[cgf(not(features = "library"))] // only compile this code if the library features isn't enabled
// these three are import statements
use cosmwasm_std::entry_point; 
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response}; 
use cw2::set_contract_version; 

// version info for migration info 
const CONTRACT_NAME: &str = "crates.io.clicker"; 
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Storing stuff in a variable called state of type State
    let state = State {
        count: msg.count, 
        // clone is important because we don't want to _give_ ownership
        owner: info.sender.clone(), 
    }; 

    // Setting contract version using a helper function 
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; 

    // Storing state in a special variable called "STATE"
    STATE.save(deps.storage, &state)?; 

    // Response back to the caller
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

// query is the special cosmwasm function to read data from contract
#[cfg_attr(not(feature = "library"), entry_point)]
// this time we need Deps (not DepsMut) because we don't need it to be mutable
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?), 
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?; 
    Ok(CountResponse { count: state.count })
}

