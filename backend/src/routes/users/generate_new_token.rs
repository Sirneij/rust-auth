use sqlx::Row;

#[derive(serde::Deserialize, Debug)]
pub struct UserEmail {
    email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SimpleUser {
    id: uuid::Uuid,
    email: String,
    first_name: String,
    last_name: String,
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    thumbnail: Option<String>,
    date_joined: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(name = "Regenerate token for a user", skip(pool, redis_pool))]
#[actix_web::post("/regenerate-token/")]
pub async fn regenerate_token(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    user_email: actix_web::web::Json<UserEmail>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
) -> actix_web::HttpResponse {
    match get_user_who_is_not_active(&pool, &user_email.email).await {
        Ok(visible_user_detail) => {
            let mut redis_con = redis_pool
                .get()
                .await
                .map_err(|e| {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
                    actix_web::HttpResponse::InternalServerError().json(
                        crate::types::ErrorResponse {
                            error: "We cannot activate your account at the moment".to_string(),
                        },
                    )
                })
                .expect("Redis connection cannot be gotten.");
            crate::utils::send_multipart_email(
                "RustAuth - Let's get you verified".to_string(),
                visible_user_detail.id,
                visible_user_detail.email,
                visible_user_detail.first_name,
                visible_user_detail.last_name,
                "verification_email.html",
                &mut redis_con,
            )
            .await
            .unwrap();
            actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
                message: "Account activation link has been sent to your email address. Kindly take action before its expiration".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found:{:#?}", e);
            actix_web::HttpResponse::NotFound().json(crate::types::ErrorResponse {
                error: "A user with this e-mail address does not exist. If you registered with this email, ensure you haven't activated it yet. You can check by logging in".to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB who isn't active yet.", skip(pool, email),fields(user_email = %email))]
async fn get_user_who_is_not_active(
    pool: &sqlx::postgres::PgPool,
    email: &String,
) -> Result<SimpleUser, sqlx::Error> {
    match sqlx::query("SELECT id, email, first_name, last_name, password, is_active, is_staff, is_superuser, date_joined, thumbnail FROM users WHERE email = $1 AND is_active=false")
        .bind(email)
        .map(|row: sqlx::postgres::PgRow| SimpleUser {
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
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}
