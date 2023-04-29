use sqlx::{postgres::PgRow, Row};

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
        Ok(id) => match get_current_user_from_db(&pool, id).await {
            Ok(user) => {
                tracing::event!(target: "backend", tracing::Level::INFO, "User retrieved from the DB.");
                actix_web::HttpResponse::Ok().json(user)
            }
            Err(e) => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "User cannot be retrieved from the DB: {:#?}", e);
                let error_message = crate::types::ErrorResponse {
                    error: "We cannot currently fulfill your request".to_string(),
                };
                actix_web::HttpResponse::InternalServerError().json(error_message)
            }
        },

        Err(e) => {
            tracing::event!(target: "session",tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            actix_web::HttpResponse::Unauthorized().json(crate::types::ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Getting current user from the DB.", skip(pool))]
async fn get_current_user_from_db(
    pool: &sqlx::postgres::PgPool,
    user_id: uuid::Uuid,
) -> Result<crate::types::UserVisible, sqlx::Error> {
    match sqlx::query("SELECT * FROM users WHERE id=$1")
        .bind(user_id)
        .map(|row: PgRow| crate::types::UserVisible {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            is_staff: row.get("is_staff"),
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
        .fetch_one(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to fetch user:{:#?}", e);
            Err(e)
        }
    }
}
