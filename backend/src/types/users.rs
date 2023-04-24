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
}
#[derive(serde::Serialize)]
pub struct LoggedInUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub is_staff: bool,
    pub is_superuser: bool,
}
