#[derive(serde::Deserialize)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(name = "Confirming change password token", skip(params, redis_pool))]
#[actix_web::get("/confirm/change-password/")]
pub async fn confirm_change_password_token(
    params: actix_web::web::Query<Parameters>,
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
                        "{}/auth/error?reason=Something unexpected happened. Kindly try again",
                        settings.frontend_url
                    ),
                ))
                .finish()
        })
        .expect("Redis connection cannot be gotten.");

    let confirmation_token = match crate::utils::verify_confirmation_token_pasetor(
        params.token.clone(),
        &mut redis_con,
        None,
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "backend",tracing::Level::ERROR, "{:#?}", e);

            return actix_web::HttpResponse::SeeOther()
                .insert_header((
                    actix_web::http::header::LOCATION,
                    format!("{}/auth/error?reason=It appears that your password request token has expired or previously used", settings.frontend_url, ),
                ))
                .finish();
        }
    };

    let issued_token = match crate::utils::issue_confirmation_token_pasetors(
        confirmation_token.user_id,
        &mut redis_con,
        Some(true),
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            return actix_web::HttpResponse::SeeOther()
                .insert_header((
                    actix_web::http::header::LOCATION,
                    format!("{}/auth/error?reason={}", settings.frontend_url, e),
                ))
                .json(crate::types::ErrorResponse {
                    error: format!("{}", e),
                });
        }
    };

    actix_web::HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "{}/auth/password/change-password?token={}",
                settings.frontend_url, issued_token
            ),
        ))
        .finish()
}
