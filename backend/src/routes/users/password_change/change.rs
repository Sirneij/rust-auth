#[derive(serde::Deserialize)]
pub struct NewPassword {
    token: String,
    password: String,
}

#[tracing::instrument(
    name = "Changing user's password",
    skip(pool, new_password, redis_pool)
)]
#[actix_web::post("/change-user-password/")]
pub async fn change_user_password(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    new_password: actix_web::web::Json<NewPassword>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
) -> actix_web::HttpResponse {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let mut redis_con = redis_pool
        .get()
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);

            actix_web::HttpResponse::SeeOther()
                .insert_header((
                    actix_web::http::header::LOCATION,
                    format!(
                        "{}/auth/error?reason=We cannot activate your account at the moment",
                        settings.frontend_url
                    ),
                ))
                .finish()
        })
        .expect("Redis connection cannot be gotten.");

    let confirmation_token = match crate::utils::verify_confirmation_token_pasetor(
        new_password.0.token,
        &mut redis_con,
        Some(true),
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "backend",tracing::Level::ERROR, "{:#?}", e);
            return actix_web::HttpResponse::BadRequest().json(crate::types::ErrorResponse {
                error: "It appears that your password request token has expired or previously used"
                    .to_string(),
            });
        }
    };

    let new_user_password = crate::utils::hash(new_password.0.password.as_bytes()).await;

    match update_user_password_in_db(&pool, &new_user_password, confirmation_token.user_id).await {
        Ok(_) => {
            tracing::event!(target: "backend",tracing::Level::INFO, "User password updated successfully");
            actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
                message: "Your password has been changed successfully. Kindly login with the new password".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target: "backend",tracing::Level::ERROR, "Failed to change user password: {:#?}", e);
            actix_web::HttpResponse::BadRequest().json(crate::types::ErrorResponse {
                error: "Sorry, we could not change your password this time. Please try again."
                    .to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Updating user password in the DB.", skip(pool, new_password))]
async fn update_user_password_in_db(
    pool: &sqlx::postgres::PgPool,
    new_password: &String,
    user_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    match sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(new_password)
        .bind(user_id)
        .execute(pool)
        .await
    {
        Ok(r) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User password has been updated successfully in the DB: {:?}", r);
            Ok(true)
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user password in the DB: {:#?}", e);
            Err(e)
        }
    }
}
