#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    pub message: String,
}
