#[tracing::instrument]
#[actix_web::get("/health-check/")]
pub async fn health_check() -> actix_web::HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Accessing health-check endpoint.");

    actix_web::HttpResponse::Ok().json(crate::types::SuccessResponse {
        message: "Application is safe and healthy.".to_string(),
    })
}
