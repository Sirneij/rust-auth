#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

pub const USER_ID_KEY: &'static str = "user_id";
pub const USER_EMAIL_KEY: &'static str = "user_email";
pub const USER_IS_STAFF_KEY: &'static str = "user_is_staff";
pub const USER_IS_SUPERUSER_KEY: &'static str = "user_is_superuser";