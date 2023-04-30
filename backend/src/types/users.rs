#[derive(serde::Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
    pub profile: UserProfile,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserVisible {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
    pub profile: UserProfile,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub phone_number: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub github_link: Option<String>,
}
#[derive(serde::Serialize)]
pub struct LoggedInUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub is_staff: bool,
    pub is_superuser: bool,
}
