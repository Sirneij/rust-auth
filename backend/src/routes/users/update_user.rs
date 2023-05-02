use sqlx::Row;

#[derive(actix_multipart::form::MultipartForm)]
pub struct UserForm {
    first_name: Option<actix_multipart::form::text::Text<String>>,
    last_name: Option<actix_multipart::form::text::Text<String>>,
    #[multipart(limit = "1 MiB")]
    thumbnail: Option<actix_multipart::form::tempfile::TempFile>,
    phone_number: Option<actix_multipart::form::text::Text<String>>,
    birth_date: Option<actix_multipart::form::text::Text<String>>,
    github_link: Option<actix_multipart::form::text::Text<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateUser {
    first_name: Option<String>,
    thumbnail: Option<String>,
    last_name: Option<String>,
}
#[derive(serde::Deserialize, Debug)]
pub struct UpdateUserProfile {
    phone_number: Option<String>,
    birth_date: Option<chrono::NaiveDate>,
    github_link: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Thumbnail {
    pub thumbnail: Option<String>,
}

#[tracing::instrument(name = "Updating an user", skip(pool, form, session))]
#[actix_web::patch("/update-user/")]
pub async fn update_users_details(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    form: actix_multipart::form::MultipartForm<UserForm>,
    session: actix_session::Session,
    s3_client: actix_web::web::Data<crate::uploads::Client>,
) -> actix_web::HttpResponse {
    let session_uuid = match crate::routes::users::logout::session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session",tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return actix_web::HttpResponse::Unauthorized().json(crate::types::ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };

    // Create a transaction object.
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

    // At first, set all fields to update to None except user_id
    let mut updated_user = UpdateUser {
        first_name: None,
        last_name: None,
        thumbnail: None,
    };

    let mut user_profile = UpdateUserProfile {
        phone_number: None,
        birth_date: None,
        github_link: None,
    };

    // If thumbnail was included for update
    if let Some(thumbnail) = &form.0.thumbnail {
        // upload temp files to s3 and then remove them
        match thumbnail.file_name.as_deref() {
            Some(name) => {
                // Ensures that file name is not empty
                if !name.is_empty() {
                    // Get user's current thumbnail from the DB
                    let user_current_thumbnail = match sqlx::query(
                        "SELECT thumbnail FROM users WHERE id=$1",
                    )
                    .bind(session_uuid)
                    .map(|row: sqlx::postgres::PgRow| Thumbnail {
                        thumbnail: row.get("thumbnail"),
                    })
                    .fetch_one(&mut *transaction)
                    .await
                    {
                        Ok(image_url) => image_url.thumbnail,
                        Err(e) => {
                            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to get user thumbnail from the DB: {:#?}", e);
                            None
                        }
                    };
                    // If there is a current image, delete it
                    if let Some(url) = user_current_thumbnail {
                        let s3_image_key = &url[url.find("media").unwrap_or(url.len())..];

                        if !s3_client.delete_file(s3_image_key).await {
                            tracing::event!(target: "backend",tracing::Level::INFO, "We could not delete the current thumbnail of user with ID: {}", session_uuid);
                        }
                    }
                    // make key prefix (make sure it ends with a forward slash)
                    let s3_key_prefix = format!("media/rust-auth/{session_uuid}/");
                    let uploaded_file = s3_client.upload(thumbnail, &s3_key_prefix).await;
                    updated_user.thumbnail = Some(uploaded_file.s3_url);
                } else {
                    tracing::event!(target: "backend",tracing::Level::INFO, "Uploaded file was empty");
                }
            }
            None => {
                tracing::event!(target: "backend",tracing::Level::INFO, "Uploaded file was null...");
            }
        }
    }

    // If first_name is updated
    if let Some(f_name) = form.0.first_name {
        updated_user.first_name = Some(f_name.0);
    }

    // If last_name is updated
    if let Some(l_name) = form.0.last_name {
        updated_user.last_name = Some(l_name.0);
    }

    // If phone_number is updated
    if let Some(phone) = form.0.phone_number {
        user_profile.phone_number = Some(phone.0);
    }
    // If birth_date is updated
    if let Some(bd) = form.0.birth_date {
        match chrono::NaiveDate::parse_from_str(&bd.0, "%Y-%m-%d") {
            Ok(d) => {
                user_profile.birth_date = Some(d);
            }
            Err(e) => {
                tracing::event!(target: "backend",tracing::Level::INFO, "Date cannot be parsed: {:#?}", e);
            }
        }
    }
    // If github_link is updated
    if let Some(gl) = form.0.github_link {
        user_profile.github_link = Some(gl.0);
    }

    // Update a user in the DB
    match update_user_in_db(&mut transaction, &updated_user, &user_profile, session_uuid).await {
        Ok(u) => u,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user in DB: {:#?}", e);
            let error_message = crate::types::ErrorResponse {
                error: format!("User could not be updated: {e}"),
            };
            return actix_web::HttpResponse::InternalServerError().json(error_message);
        }
    };

    let updated_user = match crate::utils::get_active_user_from_db(
        None,
        Some(&mut transaction),
        Some(session_uuid),
        None,
    )
    .await
    {
        Ok(user) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "User retrieved from the DB.");
            crate::types::UserVisible {
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
            }
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "User cannot be retrieved from the DB: {:#?}", e);
            let error_message = crate::types::ErrorResponse {
                error: "User was not found".to_string(),
            };
            return actix_web::HttpResponse::NotFound().json(error_message);
        }
    };

    if transaction.commit().await.is_err() {
        return actix_web::HttpResponse::InternalServerError().finish();
    }

    tracing::event!(target: "backend", tracing::Level::INFO, "User updated successfully.");
    actix_web::HttpResponse::Ok().json(updated_user)
}

#[tracing::instrument(name = "Updating user in DB.", skip(transaction))]
async fn update_user_in_db(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_to_update: &UpdateUser,
    user_profile: &UpdateUserProfile,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    match sqlx::query(
        "
        UPDATE 
            users 
        SET 
            first_name = COALESCE($1, first_name), 
            last_name = COALESCE($2, last_name), 
            thumbnail = COALESCE($3, thumbnail)
        WHERE 
            id = $4 
            AND is_active = true 
            AND (
                $1 IS NOT NULL 
                AND $1 IS DISTINCT 
                FROM 
                    first_name 
                    OR $2 IS NOT NULL 
                    AND $2 IS DISTINCT 
                FROM 
                    last_name 
                    OR $3 IS DISTINCT 
                FROM 
                    thumbnail
            )",
    )
    .bind(&user_to_update.first_name)
    .bind(&user_to_update.last_name)
    .bind(&user_to_update.thumbnail)
    .bind(user_id)
    .execute(&mut *transaction)
    .await
    {
        Ok(r) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User has been updated successfully: {:#?}", r);
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user into DB: {:#?}", e);
            return Err(e);
        }
    }

    match sqlx::query(
        "
        UPDATE 
            user_profile 
        SET 
            phone_number = NULLIF($1, ''), 
            birth_date = $2, 
            github_link = NULLIF($3, '')
        WHERE 
            user_id = $4 
            AND (
                $1 IS DISTINCT 
                FROM 
                    phone_number 
                    OR $2 IS DISTINCT 
                FROM 
                    birth_date 
                    OR $3 IS DISTINCT 
                FROM 
                    github_link
            )",
    )
    .bind(&user_profile.phone_number)
    .bind(user_profile.birth_date)
    .bind(&user_profile.github_link)
    .bind(user_id)
    .execute(&mut *transaction)
    .await
    {
        Ok(r) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User profile has been updated successfully: {:#?}", r);
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to update user profile into DB: {:#?}", e);
            return Err(e);
        }
    }

    Ok(())
}
