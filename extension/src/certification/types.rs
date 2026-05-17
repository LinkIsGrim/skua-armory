//! Certification value types.

use serde::{Deserialize, Serialize};

/// Row shape returned by `certification:list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Certification {
    pub id: String,
    pub display_name: String,
    pub document: String,
    pub description: String,
    pub perk: String,
    pub pay_bonus: i32,
    pub grant_event: String,
    pub revoke_event: String,
    pub requires: Vec<String>,
}

/// JSON file shape under `database/migrations/certifications/<id>.json`.
/// The file's stem provides the `id`; the JSON body provides the rest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CertificationFile {
    pub display_name: String,
    pub document: String,
    pub description: String,
    pub perk: String,
    pub pay_bonus: i32,
    pub grant_event: String,
    pub revoke_event: String,
    pub requires: Vec<String>,
}
