#[derive(serde::Deserialize, Debug)]
pub struct UserEmail {
    email: String,
}

#[tracing::instrument(name = "Requesting a password change", skip(pool, redis_pool))]
#[actix_web::post("/request-password-change/")]
pub async fn request_password_change(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    user_email: actix_web::web::Json<UserEmail>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
) -> actix_web::HttpResponse {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");
    match crate::utils::get_active_user_from_db(Some(&pool), None, None, Some(&user_email.0.email))
        .await
    {
        Ok(visible_user_detail) => {
            let mut redis_con = redis_pool
                .get()
                .await
                .map_err(|e| {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
                    actix_web::HttpResponse::InternalServerError().json(
                        crate::types::ErrorResponse {
                            error: "Something happened. Please try again".to_string(),
                        },
                    )
                })
                .expect("Redis connection cannot be gotten.");
            crate::utils::send_multipart_email(
                "RustAuth - Password Reset Instructions".to_string(),
                visible_user_detail.id,
                visible_user_detail.email,
                visible_user_detail.first_name,
                visible_user_detail.last_name,
                "password_reset_email.html",
                &mut redis_con,
            )
            .await
            .unwrap();
            actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
                message: "Password reset instructions have been sent to your email address. Kindly take action before its expiration".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found:{:#?}", e);
            actix_web::HttpResponse::NotFound().json(crate::types::ErrorResponse {
                error: format!("An active user with this e-mail address does not exist. If you registered with this email, ensure you have activated your account. You can check by logging in. If you have not activated it, visit {}/auth/regenerate-token to regenerate the token that will allow you activate your account.", settings.frontend_url),
            })
        }
    }
}
