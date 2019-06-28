use actix_web::{web, HttpResponse};
use postgres::Connection;
use std::sync::Mutex;
use std::thread;

use crate::communication::send_email::send_email;
use crate::commontypes::ForgotPassword;

pub fn forgot_password(forgot_password: web::Json<ForgotPassword>, state: web::Data<Mutex<Connection>>) -> HttpResponse {
    match state.lock() {
        Ok(pg_instance) => {
            let user = pg_instance.query("SELECT * FROM rust_users where username = $1",
                                         &[&forgot_password.username]);
            match user {
                Ok(users_db) => {
                    if users_db.len() == 0 {
                        HttpResponse::BadRequest().body("check username")
                    } else {
                        let email: String = users_db.get(0).get("email");
                        let cloned_email = email.clone();
                        thread::spawn(move || {
                            send_email(cloned_email);
                        });
                        return HttpResponse::Ok().body(email);
                    }
                },
                Err(error) => {
                    println!("error while retrieving {}", error);
                    return HttpResponse::InternalServerError().body("")
                }
            }
        },
        Err(error) => {
            println!("unable to get DB instance {}", error);
            HttpResponse::InternalServerError().body("")
        }
    }
}