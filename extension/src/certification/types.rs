//! Certification value types.

use arma_rs::IntoArma;
use serde::Deserialize;

/// Row shape returned by `certification:list`.
#[derive(Debug, Clone, PartialEq, Eq, IntoArma)]
pub struct Certification {
    pub id: String,
    pub display_name: String,
    pub document: String,
    pub grant_event: String,
    pub revoke_event: String,
}

/// JSON file shape under `database/migrations/certifications/<id>.json`.
/// The file's stem provides the `id`; the JSON body provides the rest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CertificationFile {
    pub display_name: String,
    pub document: String,
    pub grant_event: String,
    pub revoke_event: String,
}
