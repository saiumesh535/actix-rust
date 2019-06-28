use actix_web::{Scope, web};

mod login;
use login::login;

mod signup;
use signup::signup;

mod forgot_password;
use forgot_password::forgot_password;

pub fn auth_routes() -> Scope {
    web::scope("/auth/")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/signup").route(web::post().to(signup)))
        .service(web::resource("/forgotpassword").route(web::post().to(forgot_password)))
}
