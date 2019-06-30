use actix_web::{web, HttpResponse};

use std::sync::Mutex;
use chrono::{Local, Duration};
use std::convert::AsRef;
use jsonwebtoken::{ encode, Header };
use serde_derive::{Deserialize, Serialize};
use crate::commontypes::{ ErrorMessage };
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;


#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
struct LoginTokenClaims {
    data: String,
    exp: i64
}

#[derive(Serialize)]
struct Token {
    token: String
}

pub fn login(login: web::Json<Login>, state: web::Data<Mutex<PooledConnection<PostgresConnectionManager>>>) -> HttpResponse {
    match state.lock() {
        Ok(pg_instance) => {
            match pg_instance.query("select * from rust_users where username = $1", &[&login.username]) {
                Ok(users) => {
                    if users.is_empty() {
                        return HttpResponse::BadRequest().json(ErrorMessage {
                            message: String::from("check username or password")
                        });
                    }
                    let my_claims = LoginTokenClaims {
                        data: login.username.clone(),
                        exp: (Local::now() + Duration::hours(24)).timestamp()
                    };
                    match encode(&Header::default(), &my_claims, "thisissecret".as_ref()) {
                        Ok(token) => {
                            return  HttpResponse::Ok().json(Token {
                                    token: token
                                })
                        }, Err(jwt_error) => {
                            println!("error has occurred {}", jwt_error);
                            return HttpResponse::InternalServerError().body("")
                        }
                    }
                },
                Err(err) => {
                    println!("error has occurred {}", err);
                    return HttpResponse::InternalServerError().body("")
                }
            };
        },
        Err(db_er) => {
            println!("unable to get DB instance {}", db_er);
            return HttpResponse::InternalServerError().body("")
        }
    }
}