#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}

pub const USER_ID_KEY: &str = "user_id";
pub const USER_EMAIL_KEY: &str = "user_email";
pub const USER_IS_STAFF_KEY: &str = "user_is_staff";
pub const USER_IS_SUPERUSER_KEY: &str = "user_is_superuser";
