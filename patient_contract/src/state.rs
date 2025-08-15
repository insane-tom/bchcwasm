use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Patient {
    pub name: String,
    pub age: u8,
    pub disease: String,
}

// Stockage des patients : patient_id -> Patient
pub const PATIENTS: Map<u64, Patient> = Map::new("patients");

// Stockage des accès : patient_id -> liste d'adresses autorisées
pub const ACCESS: Map<u64, Vec<Addr>> = Map::new("access");

// Compteur pour générer des patient_id uniques
pub const PATIENT_COUNT: Item<u64> = Item::new("patient_count");