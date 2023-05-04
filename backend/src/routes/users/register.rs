use sqlx::Row;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateNewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name = "Adding a new user",
skip( pool, new_user, redis_pool),
fields(
    new_user_email = %new_user.email,
    new_user_first_name = %new_user.first_name,
    new_user_last_name = %new_user.last_name
))]
#[actix_web::post("/register/")]
pub async fn register_user(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    new_user: actix_web::web::Json<NewUser>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
) -> actix_web::HttpResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(
                crate::types::ErrorResponse {
                    error: "Something unexpected happend. Kindly try again.".to_string(),
                },
            );
        }
    };
    let hashed_password = crate::utils::hash(new_user.0.password.as_bytes()).await;

    let create_new_user = CreateNewUser {
        password: hashed_password,
        email: new_user.0.email,
        first_name: new_user.0.first_name,
        last_name: new_user.0.last_name,
    };

    let user_id = match insert_created_user_into_db(&mut transaction, &create_new_user).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            let error_message = if e
                .as_database_error()
                .unwrap()
                .code()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                == 23505
            {
                crate::types::ErrorResponse {
                    error: "A user with that email address already exists".to_string(),
                }
            } else {
                crate::types::ErrorResponse {
                    error: "Error inserting user into the database".to_string(),
                }
            };
            return actix_web::HttpResponse::BadRequest().json(error_message);
        }
    };

    // send confirmation email to the new user.
    let mut redis_con = redis_pool
        .get()
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            actix_web::HttpResponse::InternalServerError().json(crate::types::ErrorResponse {
                error: "We cannot activate your account at the moment".to_string(),
            })
        })
        .expect("Redis connection cannot be gotten.");

    crate::utils::send_multipart_email(
        "RustAuth - Let's get you verified".to_string(),
        user_id,
        create_new_user.email,
        create_new_user.first_name,
        create_new_user.last_name,
        "verification_email.html",
        &mut redis_con,
    )
    .await
    .unwrap();

    if transaction.commit().await.is_err() {
        return actix_web::HttpResponse::InternalServerError().finish();
    }

    tracing::event!(target: "backend", tracing::Level::INFO, "User created successfully.");
    actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
        message: "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires".to_string(),
    })
}

#[tracing::instrument(name = "Inserting new user into DB.", skip(transaction, new_user),fields(
    new_user_email = %new_user.email,
    new_user_first_name = %new_user.first_name,
    new_user_last_name = %new_user.last_name
))]
async fn insert_created_user_into_db(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    new_user: &CreateNewUser,
) -> Result<uuid::Uuid, sqlx::Error> {
    let user_id = match sqlx::query(
        "INSERT INTO users (email, password, first_name, last_name) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&new_user.email)
    .bind(&new_user.password)
    .bind(&new_user.first_name)
    .bind(&new_user.last_name)
    .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid{
        row.get("id")
   })
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            return Err(e);
        }
    };

    match sqlx::query(
        "INSERT INTO user_profile (user_id) 
                VALUES ($1) 
            ON CONFLICT (user_id) 
            DO NOTHING
            RETURNING user_id",
    )
    .bind(user_id)
    .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("user_id") })
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(id) => {
            tracing::event!(target: "sqlx",tracing::Level::INFO, "User profile created successfully {}.", id);
            Ok(id)
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to insert user's profile into DB: {:#?}", e);
            Err(e)
        }
    }
}
