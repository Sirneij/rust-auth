#[tracing::instrument(
    name = "Accessing retrieving current user endpoint.",
    skip(pool, session)
)]
#[actix_web::get("/current-user/")]
pub async fn get_current_user(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    session: actix_session::Session,
) -> actix_web::HttpResponse {
    match crate::routes::users::logout::session_user_id(&session).await {
        Ok(id) => {
            match crate::utils::get_active_user_from_db(Some(&pool), None, Some(id), None).await {
                Ok(user) => {
                    tracing::event!(target: "backend", tracing::Level::INFO, "User retrieved from the DB.");
                    actix_web::HttpResponse::Ok().json(crate::types::UserVisible {
                        id: user.id,
                        email: user.email,
                        first_name: user.first_name,
                        last_name: user.last_name,
                        is_active: user.is_active,
                        is_staff: user.is_staff,
                        is_superuser: user.is_superuser,
                        date_joined: user.date_joined,
                        thumbnail: user.thumbnail,
                        profile: crate::types::UserProfile {
                            id: user.profile.id,
                            user_id: user.profile.user_id,
                            phone_number: user.profile.phone_number,
                            birth_date: user.profile.birth_date,
                            github_link: user.profile.github_link,
                        },
                    })
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "User cannot be retrieved from the DB: {:#?}", e);
                    let error_message = crate::types::ErrorResponse {
                        error: "User was not found".to_string(),
                    };
                    actix_web::HttpResponse::NotFound().json(error_message)
                }
            }
        }

        Err(e) => {
            tracing::event!(target: "session",tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            actix_web::HttpResponse::Unauthorized().json(crate::types::ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            })
        }
    }
}
