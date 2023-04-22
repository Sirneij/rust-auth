#[actix_web::get("/health-check/")]
pub async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json("Application is safe and healthy.")
}
