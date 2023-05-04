use crate::helpers::spawn_app;
use fake::faker::{
    internet::en::SafeEmail,
    name::en::{FirstName, LastName, NameWithTitle},
};
use fake::Fake;
use sqlx::Row;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser<'a> {
    email: &'a str,
    password: String,
    first_name: String,
    last_name: String,
}

#[sqlx::test]
async fn test_register_user_success(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Request data
    let email: String = SafeEmail().fake();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let password = NameWithTitle().fake();
    let new_user = NewUser {
        email: &email,
        password,
        first_name,
        last_name,
    };

    let response = app
        .api_client
        .post(&format!("{}/users/register/", &app.address))
        .json(&new_user)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved_user = sqlx::query(
        "SELECT 
            u.id AS u_id, 
            u.email AS u_email, 
            u.password AS u_password, 
            u.first_name AS u_first_name, 
            u.last_name AS u_last_name, 
            u.is_active AS u_is_active, 
            u.is_staff AS u_is_staff, 
            u.is_superuser AS u_is_superuser, 
            u.thumbnail AS u_thumbnail, 
            u.date_joined AS u_date_joined, 
            p.id AS p_id, 
            p.user_id AS p_user_id, 
            p.phone_number AS p_phone_number, 
            p.birth_date AS p_birth_date, 
            p.github_link AS p_github_link 
        FROM 
            users u 
            LEFT JOIN user_profile p ON p.user_id = u.id
        WHERE 
            u.is_active=false AND u.email=$1
    ",
    )
    .bind(&email)
    .map(|row: sqlx::postgres::PgRow| backend::types::User {
        id: row.get("u_id"),
        email: row.get("u_email"),
        first_name: row.get("u_first_name"),
        password: row.get("u_password"),
        last_name: row.get("u_last_name"),
        is_active: row.get("u_is_active"),
        is_staff: row.get("u_is_staff"),
        is_superuser: row.get("u_is_superuser"),
        thumbnail: row.get("u_thumbnail"),
        date_joined: row.get("u_date_joined"),
        profile: backend::types::UserProfile {
            id: row.get("p_id"),
            user_id: row.get("p_user_id"),
            phone_number: row.get("p_phone_number"),
            birth_date: row.get("p_birth_date"),
            github_link: row.get("p_github_link"),
        },
    })
    .fetch_one(&pool)
    .await
    .expect("msg");

    assert_eq!(saved_user.is_active, false);
    assert_eq!(saved_user.email, email);
    assert_eq!(saved_user.thumbnail, None);
    assert_eq!(saved_user.profile.user_id, saved_user.id);
    assert_eq!(saved_user.profile.phone_number, None)
}
#[sqlx::test]
async fn test_register_user_failure_email(pool: sqlx::postgres::PgPool) {
    let app = spawn_app(pool.clone()).await;

    // First request data
    let email = "backend@api.com".to_string();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let password = NameWithTitle().fake();
    let new_user_one = NewUser {
        email: &email,
        password,
        first_name,
        last_name,
    };

    let response_one = app
        .api_client
        .post(&format!("{}/users/register/", &app.address))
        .json(&new_user_one)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response_one.status().is_success());

    // First request data
    let email = "backend@api.com".to_string();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let password = NameWithTitle().fake();
    let new_user_two = NewUser {
        email: &email,
        password,
        first_name,
        last_name,
    };

    let response_two = app
        .api_client
        .post(&format!("{}/users/register/", &app.address))
        .json(&new_user_two)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response_two.status().is_client_error());

    let error_response = response_two
        .json::<backend::types::ErrorResponse>()
        .await
        .expect("Cannot get user response");

    assert_eq!(
        error_response.error,
        "A user with that email address already exists"
    );
}
