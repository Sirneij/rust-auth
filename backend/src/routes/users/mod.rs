mod confirm_registration;
mod current_user;
mod login;
mod logout;
mod register;
mod update_user;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/users")
            .service(register::register_user)
            .service(confirm_registration::confirm)
            .service(login::login_user)
            .service(current_user::get_current_user)
            .service(update_user::update_users_details)
            .service(logout::log_out),
    );
}
