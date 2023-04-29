#[tracing::instrument(name = "Log out user", skip(session))]
#[actix_web::post("/logout/")]
pub async fn log_out(session: actix_session::Session) -> actix_web::HttpResponse {
    match session_user_id(&session).await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "Users retrieved from the DB.");
            session.purge();
            actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
                message: "You have successfully logged out".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target: "backend",tracing::Level::ERROR, "Failed to get user from session: {:#?}", e);
            actix_web::HttpResponse::BadRequest().json(crate::types::ErrorResponse {
                error:
                    "We currently have some issues. Kindly try again and ensure you are logged in"
                        .to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Get user_id from session.", skip(session))]
pub async fn session_user_id(session: &actix_session::Session) -> Result<uuid::Uuid, String> {
    match session.get(crate::types::USER_ID_KEY) {
        Ok(user_id) => match user_id {
            None => Err("You are not authenticated".to_string()),
            Some(id) => Ok(id),
        },
        Err(e) => Err(format!("{e}")),
    }
}
