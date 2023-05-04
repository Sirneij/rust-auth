use crate::helpers::spawn_app;
use fake::Fake;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[sqlx::test]
async fn test_login_user_failure_bad_request(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: fake::faker::name::en::NameWithTitle().fake(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_client_error());

    let error_response = login_response
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(error_response.error, "Email and password do not match");
}

#[sqlx::test]
async fn test_login_user_failure_notfound(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Act - Part 1 - Login
    let login_body = LoginUser {
        email: fake::faker::internet::en::SafeEmail().fake(),
        password: app.test_user.password.clone(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_client_error());

    let error_response = login_response
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(error_response.error, "A user with these details does not exist. If you registered with these details, ensure you activate your account by clicking on the link sent to your e-mail address");
}

#[sqlx::test]
async fn test_login_user_success(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

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

    // Check response
    let response = login_response
        .json::<backend::types::UserVisible>()
        .await
        .expect("Cannot get user response");

    assert_eq!(response.email, app.test_user.email);
    assert!(response.is_active);
    assert_eq!(response.id, response.profile.user_id);
}
