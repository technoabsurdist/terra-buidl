// main contract for contract terra project

#[cfg(not(features = "library"))]
use cosmwasm_std::entry_point; 
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult}; 
use cw2::set_contract_version; 

use crate::error::ContractError; 
use crate::msg::{CountResponse, InstantiateMsg, QueryMsg, ScoreResponse, ExecuteMsg}; 
use crate::state::{State, STORAGE}; 

#[cfg(not(features = "library"))] // only compile this code if the library features isn't enabled
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
        owner: info.sender.clone(), 
        scores: vec![],
    }; 

    // Setting contract version using a helper function 
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; 

    // Storing state in a special variable called "STORAGE"
    STORAGE.save(deps.storage, &state)?; 

    // Response back to the caller
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string())
        .add_attribute("scores", "".to_string()))
}

// Main execute message handler, we need 'info' as a paramater too
#[cfg_attr(not(features = "library"), entry_point)]
pub fn execute(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpsertScore { score } => try_upsert_score(deps, info, score), 
    }
}

fn try_upsert_score(
    deps: DepsMut, 
    info: MessageInfo, 
    score: u16,
) -> Result<Response, ContractError> {
    let mut state = STORAGE.load(deps.storage)?;
    let sender = info.sender.clone(); 
    let scores = &mut state.scores; 
    let index = scores.iter().position(|(s,_)| s == &sender); 
    match index {
        Some(i) => {
            scores[i].1 = score; 
        }, 
        None => {
            scores.push((sender.clone(), score)); 
        }
    }
    STORAGE.save(deps.storage, &state)?; 
    Ok(Response::new()
        .add_attribute("method", "upsert")
        .add_attribute("player", info.sender)
        .add_attribute("score", score.to_string()))
}



// query is the special cosmwasm function to read data from contract
#[cfg_attr(not(feature = "library"), entry_point)]
// this time we need Deps (not DepsMut) because we don't need it to be mutable
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?), 
        QueryMsg::GetScores {} => to_binary(&query_scores(deps)?), 
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STORAGE.load(deps.storage)?; 
    Ok(CountResponse { count: state.count })
}

fn query_scores(deps: Deps) -> StdResult<ScoreResponse> {
    let state = STORAGE.load(deps.storage)?; 
    Ok(ScoreResponse { scores: state.scores })
    
}