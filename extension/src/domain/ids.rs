//! Identifier types for domain entities.

use std::fmt::Display;

use arma_rs::{FromArma, IntoArma};
use tokio_postgres::types::{IsNull, ToSql, Type, private::BytesMut};

/// Steam ID for a player. Stored in Postgres as `BIGINT`; the underlying
/// representation is `u64` so the full Steam ID space is preserved without
/// signed-overflow shenanigans on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    #[must_use] 
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Casts the underlying `u64` to `i64` for use as a Postgres `BIGINT`
    /// parameter. Reinterprets the bits; round-trips losslessly through the
    /// DB column as long as both sides use the same convention.
    #[must_use] 
    pub const fn as_i64(self) -> i64 {
        self.0.cast_signed()
    }

    /// Reverse of [`as_i64`].
    #[must_use] 
    pub const fn from_i64(raw: i64) -> Self {
        Self(raw.cast_unsigned())
    }

    #[must_use] 
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for PlayerId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<PlayerId> for u64 {
    fn from(value: PlayerId) -> Self {
        value.0
    }
}

impl FromArma for PlayerId {
    fn from_arma(s: String) -> Result<Self, arma_rs::FromArmaError> {
        // Arma sometimes wraps strings in quotes; trim them before parsing.
        let trimmed = s.trim_matches('"');
        trimmed
            .parse::<u64>()
            .map(PlayerId)
            .map_err(|e| arma_rs::FromArmaError::Custom(format!("invalid PlayerId: {e}")))
    }
}

impl IntoArma for PlayerId {
    fn to_arma(&self) -> arma_rs::Value {
        arma_rs::Value::String(self.0.to_string())
    }
}

impl ToSql for PlayerId {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <i64 as ToSql>::to_sql(&self.as_i64(), ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <i64 as ToSql>::accepts(ty)
    }

    tokio_postgres::types::to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use arma_rs::Value;

    #[test]
    fn display_emits_decimal() {
        assert_eq!(
            PlayerId::new(76561198000000000).to_string(),
            "76561198000000000"
        );
    }

    #[test]
    fn from_arma_accepts_bare_digits() {
        let parsed = PlayerId::from_arma("76561198000000000".into()).unwrap();
        assert_eq!(parsed, PlayerId::new(76561198000000000));
    }

    #[test]
    fn from_arma_accepts_quoted_digits() {
        let parsed = PlayerId::from_arma("\"76561198000000000\"".into()).unwrap();
        assert_eq!(parsed, PlayerId::new(76561198000000000));
    }

    #[test]
    fn from_arma_rejects_non_numeric() {
        assert!(PlayerId::from_arma("nope".into()).is_err());
    }

    #[test]
    fn from_arma_rejects_negative() {
        assert!(PlayerId::from_arma("-1".into()).is_err());
    }

    #[test]
    fn into_arma_emits_string() {
        let value = PlayerId::new(42).to_arma();
        assert_eq!(value, Value::String("42".into()));
    }

    #[test]
    fn as_i64_round_trip_at_u64_max() {
        let p = PlayerId::new(u64::MAX);
        // u64::MAX bit-cast to i64 is -1; round-trip via from_i64 must recover MAX.
        assert_eq!(p.as_i64(), -1);
        assert_eq!(PlayerId::from_i64(p.as_i64()), p);
    }

    #[test]
    fn as_i64_round_trip_for_typical_steam_id() {
        let p = PlayerId::new(76561198000000000);
        assert_eq!(PlayerId::from_i64(p.as_i64()), p);
    }

    #[test]
    fn u64_conversions_bidirectional() {
        let p: PlayerId = 123u64.into();
        let raw: u64 = p.into();
        assert_eq!(raw, 123);
    }
}
