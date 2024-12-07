use std::error::Error;

use jsonwebtoken::{
    decode,
    jwk::{AlgorithmParameters, CommonParameters, EllipticCurve, Jwk, OctetKeyPairParameters},
    Algorithm, DecodingKey, TokenData, Validation,
};

use crate::{r#const::EDDSA_JWK_X, entity::Claims};

/// Decode key encoded with `OctetKeyPair` key type and `Ed25519` curve, an example of EdDSA
/// algorithm implementation
///
/// * `jwt`: Json Web Token to decode
pub fn decode_okp_ed25519(jwt: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let mut validation = Validation::new(Algorithm::EdDSA);
    // We are using expired key here, so activating this validation is bound to fail
    validation.validate_exp = false;

    let msg = decode::<Claims>(
        jwt,
        &DecodingKey::from_jwk(&Jwk {
            common: CommonParameters::default(),
            algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                curve: EllipticCurve::Ed25519,
                x: EDDSA_JWK_X.into(),
                ..Default::default()
            }),
        })?,
        &validation,
    )?;

    Ok(msg)
}
