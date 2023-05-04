use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[sqlx::test]
async fn test_update_user_failure_not_logged_in(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // multipart form
    let form = reqwest::multipart::Form::new()
        .text("github_link", "https://github.com/Sirneij")
        .text("phone_number", "+2348135459073");

    let update_user_response = app
        .api_client
        .patch(&format!("{}/users/update-user/", &app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = update_user_response
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(
        response.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}

#[sqlx::test]
async fn test_update_user_success(pool: sqlx::postgres::PgPool) {
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

    // multipart form
    let form = reqwest::multipart::Form::new()
        .text("github_link", "https://github.com/Sirneij")
        .text("phone_number", "+2348135459073");

    let update_user_response = app
        .api_client
        .patch(&format!("{}/users/update-user/", &app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response
    let response = update_user_response
        .json::<backend::types::UserVisible>()
        .await
        .expect("Cannot get user response");

    assert_eq!(response.email, app.test_user.email);
    assert!(response.is_active);
    assert_eq!(response.id, response.profile.user_id);
    assert_eq!(
        response.profile.github_link,
        Some("https://github.com/Sirneij".to_string())
    );
    assert_eq!(
        response.profile.phone_number,
        Some("+2348135459073".to_string())
    );
}
