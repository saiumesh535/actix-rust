use actix_web::{web, HttpResponse};

use std::sync::Mutex;
use crate::auth::login::Login;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;


pub fn signup(login: web::Json<Login>, state: web::Data<Mutex<PooledConnection<PostgresConnectionManager>>>) -> HttpResponse {
    match state.lock() {
        Ok(pg_instance) => {
            let users = pg_instance.query("INSERT INTO rust_users (username, password) VALUES ($1, $2)",  &[&login.username, &login.password]);
            match users {
                Ok(_) => {
                    return HttpResponse::Ok().body("signed up");
                }, Err(error) => {
                    println!("insert error {}", error);
                    return HttpResponse::InternalServerError().body("")
                }
            }
        }, Err(error) => {
            println!("unable to get DB instance {}", error);
            return HttpResponse::InternalServerError().body("")
        }
    }
}