use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePatient { name: String, age: u8, disease: String },
    GrantAccess { patient_id: u64, addr: String },
    RevokeAccess { patient_id: u64, addr: String },
    // Lecture sécurisée via execute (le sender est authentifié)
    GetPatient { patient_id: u64 },
}

// Laisse vide (ou supprime) si tu n’as pas de queries publiques
#[cw_serde]
pub enum QueryMsg {}