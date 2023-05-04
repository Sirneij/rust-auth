use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserEmail {
    email: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser<'a> {
    email: &'a str,
    password: String,
    first_name: String,
    last_name: String,
}

#[sqlx::test]
async fn test_regenerate_token_failure(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // First login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    let user_email = UserEmail {
        email: app.test_user.email.clone(),
    };

    // Then get current user
    let res = app
        .api_client
        .post(&format!("{}/users/regenerate-token/", &app.address))
        .json(&user_email)
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = res
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(
        response.error,
        "A user with this e-mail address does not exist. If you registered with this email, ensure you haven't activated it yet. You can check by logging in"
    );
}

#[sqlx::test]
async fn test_regenerate_token_success(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Request data
    sqlx::query(
        "INSERT INTO users (email, password, first_name, last_name, is_active, is_staff, is_superuser) 
        VALUES ($1, $2, $3, $4, false, true, true)"
    )
    .bind("email@example.com")
    .bind("password_hash")
    .bind("first_name")
    .bind("last_name")
    .execute(&pool)
    .await
    .expect("Failed to store test user.");

    let user_email = UserEmail {
        email: "email@example.com".to_string(),
    };

    // Then get current user
    let get_user_response = app
        .api_client
        .post(&format!("{}/users/regenerate-token/", &app.address))
        .json(&user_email)
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = get_user_response
        .json::<backend::types::SuccessResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(response.message, "Account activation link has been sent to your email address. Kindly take action before its expiration");
}
