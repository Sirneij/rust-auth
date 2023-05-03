mod change;
mod confirm_change_request;
mod request_change;

pub fn password_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/password-change")
            .service(request_change::request_password_change)
            .service(confirm_change_request::confirm_change_password_token)
            .service(change::change_user_password),
    );
}
