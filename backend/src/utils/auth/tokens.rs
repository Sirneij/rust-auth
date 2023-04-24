use argon2::password_hash::rand_core::{OsRng, RngCore};
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4, Local};

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

/// Issues a pasetor token to a user. The token has the user's id encoded.
/// A session_key is also encoded. This key is used to destroy the token
/// as soon as it's been verified. Depending on its usage, the token issued
/// has at most an hour to live. Which means, it is destroyed after its time-to-live.
#[tracing::instrument(name = "Issue pasetors token", skip(redis_connection))]
pub async fn issue_confirmation_token_pasetors(
    user_id: uuid::Uuid,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_for_password_change: Option<bool>,
) -> Result<String, deadpool_redis::redis::RedisError> {
    // I just generate 128 bytes of random data for the session key
    // from something that is cryptographically secure (rand::CryptoRng)
    //
    // You don't necessarily need a random value, but you'll want something
    // that is sufficiently not able to be guessed (you don't want someone getting
    // an old token that is supposed to not be live, and being able to get a valid
    // token from that).
    let session_key: String = {
        let mut buff = [0_u8; 128];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };

    let redis_key = {
        if is_for_password_change.is_some() {
            format!(
                "{}{}is_for_password_change",
                SESSION_KEY_PREFIX, session_key
            )
        } else {
            format!("{}{}", SESSION_KEY_PREFIX, session_key)
        }
    };

    redis_connection
        .set(
            redis_key.clone(),
            // I just want to validate that the key exists to indicate the session is "live".
            String::new(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (set): {}", e);
            e
        })?;

    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let current_date_time = chrono::Local::now();
    let dt = {
        if is_for_password_change.is_some() {
            current_date_time + chrono::Duration::hours(1)
        } else {
            current_date_time + chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };

    let time_to_live = {
        if is_for_password_change.is_some() {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };

    redis_connection
        .expire(
            redis_key.clone(),
            time_to_live.num_seconds().try_into().unwrap(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (expiry): {}", e);
            e
        })?;

    let mut claims = Claims::new().unwrap();
    // Set custom expiration, default is 1 hour
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional("user_id", serde_json::json!(user_id))
        .unwrap();
    claims
        .add_additional("session_key", serde_json::json!(session_key))
        .unwrap();

    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();
    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .unwrap())
}

/// Verifies and destroys a token. A token is destroyed immediately
/// it has successfully been verified and all encoded data extracted.
/// Redis is used for such destruction.
#[tracing::instrument(name = "Verify pasetors token", skip(token, redis_connection))]
pub async fn verify_confirmation_token_pasetor(
    token: String,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_password: Option<bool>,
) -> Result<crate::types::ConfirmationToken, String> {
    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)
        .map_err(|e| format!("TokenValiation: {}", e))?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .map_err(|e| format!("Pasetor: {}", e))?;
    let claims = trusted_token.payload_claims().unwrap();

    let uid = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    match serde_json::from_value::<String>(uid) {
        Ok(uuid_string) => match uuid::Uuid::parse_str(&uuid_string) {
            Ok(user_uuid) => {
                let sss_key =
                    serde_json::to_value(claims.get_claim("session_key").unwrap()).unwrap();
                let session_key = match serde_json::from_value::<String>(sss_key) {
                    Ok(session_key) => session_key,
                    Err(e) => return Err(format!("{}", e)),
                };

                let redis_key = {
                    if is_password.is_some() {
                        format!(
                            "{}{}is_for_password_change",
                            SESSION_KEY_PREFIX, session_key
                        )
                    } else {
                        format!("{}{}", SESSION_KEY_PREFIX, session_key)
                    }
                };

                if redis_connection
                    .get::<_, Option<String>>(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?
                    .is_none()
                {
                    return Err("Token has been used or expired.".to_string());
                }
                redis_connection
                    .del(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?;
                Ok(crate::types::ConfirmationToken { user_id: user_uuid })
            }
            Err(e) => Err(format!("{}", e)),
        },

        Err(e) => Err(format!("{}", e)),
    }
}
