mod register;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/users").service(register::register_user));
}
