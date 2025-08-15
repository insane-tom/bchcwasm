use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
mod msg;
mod state;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Patient, ACCESS, PATIENTS, PATIENT_COUNT};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: crate::msg::InstantiateMsg,
) -> StdResult<Response> {
    PATIENT_COUNT.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreatePatient { name, age, disease } => {
            execute_create_patient(deps, info, name, age, disease)
        }
        ExecuteMsg::GrantAccess { patient_id, addr } => {
            execute_grant_access(deps, info, patient_id, addr)
        }
        ExecuteMsg::RevokeAccess { patient_id, addr } => {
            execute_revoke_access(deps, info, patient_id, addr)
        }
        ExecuteMsg::GetPatient { patient_id } => execute_get_patient(deps, info, patient_id),
    }
}

fn execute_create_patient(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    age: u8,
    disease: String,
) -> StdResult<Response> {
    let mut patient_id = PATIENT_COUNT.may_load(deps.storage)?.unwrap_or(0);
    patient_id += 1;

    let patient = Patient { name, age, disease };
    PATIENTS.save(deps.storage, patient_id, &patient)?;
    // Le créateur a accès par défaut
    ACCESS.save(deps.storage, patient_id, &vec![info.sender.clone()])?;
    PATIENT_COUNT.save(deps.storage, &patient_id)?;

    Ok(Response::new()
        .add_attribute("action", "create_patient")
        .add_attribute("patient_id", patient_id.to_string()))
}

fn execute_grant_access(
    deps: DepsMut,
    info: MessageInfo,
    patient_id: u64,
    addr: String,
) -> StdResult<Response> {
    let mut access = ACCESS.load(deps.storage, patient_id)?;
    let new_addr = deps.api.addr_validate(&addr)?;

    if !access.contains(&info.sender) {
        return Err(StdError::generic_err(
            "Vous n'êtes pas autorisé à gérer cet accès",
        ));
    }

    if !access.contains(&new_addr) {
        access.push(new_addr);
        ACCESS.save(deps.storage, patient_id, &access)?;
    }

    Ok(Response::new()
        .add_attribute("action", "grant_access")
        .add_attribute("patient_id", patient_id.to_string())
        .add_attribute("granted", addr))
}

fn execute_revoke_access(
    deps: DepsMut,
    info: MessageInfo,
    patient_id: u64,
    addr: String,
) -> StdResult<Response> {
    let mut access = ACCESS.load(deps.storage, patient_id)?;
    let remove_addr = deps.api.addr_validate(&addr)?;

    if !access.contains(&info.sender) {
        return Err(StdError::generic_err(
            "Vous n'êtes pas autorisé à gérer cet accès",
        ));
    }

    access.retain(|a| a != &remove_addr);
    ACCESS.save(deps.storage, patient_id, &access)?;

    Ok(Response::new()
        .add_attribute("action", "revoke_access")
        .add_attribute("patient_id", patient_id.to_string())
        .add_attribute("revoked", addr))
}

// ⚠️ Lecture sécurisée via execute (info.sender authentifié par la chaîne)
fn execute_get_patient(deps: DepsMut, info: MessageInfo, patient_id: u64) -> StdResult<Response> {
    let patient = PATIENTS.load(deps.storage, patient_id)?;
    let access_list = ACCESS.load(deps.storage, patient_id)?;

    if !access_list.contains(&info.sender) {
        return Err(StdError::generic_err("Accès non autorisé"));
    }

    // Pour l’instant on renvoie en clair dans les attributs (logs)
    Ok(Response::new()
        .add_attribute("action", "get_patient")
        .add_attribute("patient_id", patient_id.to_string())
        .add_attribute("name", patient.name)
        .add_attribute("age", patient.age.to_string())
        .add_attribute("disease", patient.disease))
}

// Aucune query publique (tout passe par execute pour l’auth)
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err(
        "Aucune query publique. Utilisez ExecuteMsg::GetPatient.",
    ))
}