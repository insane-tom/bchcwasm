use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use serde::{Deserialize, Serialize};

// Pas de stockage, juste un instantiate vide
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

// Définition du type de requête
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetGreeting {},
}

// Structure de la réponse
#[derive(Serialize, Deserialize)]
pub struct QueryResp {
    pub message: String,
}

// Fonction query qui renvoie toujours "Hello World"
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let resp = QueryResp {
        message: "Hello World".to_string(),
    };
    to_binary(&resp)
}
