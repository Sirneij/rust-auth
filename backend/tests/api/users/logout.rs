use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[sqlx::test]
async fn test_logout_failure(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Test logout user
    let logout_response = app
        .api_client
        .post(&format!("{}/users/logout/", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = logout_response
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(
        response.error,
        "We currently have some issues. Kindly try again and ensure you are logged in"
    );
}

#[sqlx::test]
async fn test_logout_success(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // First login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    // Check that there is cookie present
    let headers = login_response.headers();
    assert!(headers.get("set-cookie").is_some());
    let cookie_str = headers.get("set-cookie").unwrap().to_str().unwrap();
    assert!(cookie_str.contains("sessionid="));

    // Then logout user
    let logout_response = app
        .api_client
        .post(&format!("{}/users/logout/", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = logout_response
        .json::<backend::types::SuccessResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(response.message, "You have successfully logged out");
}
