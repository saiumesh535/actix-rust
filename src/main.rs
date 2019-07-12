extern crate failure;

use actix_files::Files;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, Error, Result};
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};

use futures::{future::ok, Future};
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use std::sync::Mutex;

#[allow(dead_code)]
mod redirect;

mod communication;

mod auth;
use actix_web::middleware::errhandlers::ErrorHandlers;
use auth::auth_routes;

mod commontypes;

// Connects to the Postgres Database
fn connect_pg() -> PooledConnection<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:postgres@localhost:5432/actix-web",
        TlsMode::None,
    )
    .unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    let connection = pool.get().unwrap();
    return connection;
}

fn index_async(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap())))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = HttpResponse::build(res.status())
        .body(format!("request {} not found!!", res.request().path()))
        .into_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}

fn main() -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(connect_pg()));
    HttpServer::new(move || {
        App::new()
            .wrap(redirect::CheckLogin)
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, not_found)
                    .handler(http::StatusCode::METHOD_NOT_ALLOWED, not_found),
            )
            .register_data(state.clone())
            .service(auth_routes())
            .service(web::resource("/hello/{name}").route(web::get().to_async(index_async)))
            .service(Files::new("/", "client/").index_file("index.html"))
    })
    .bind("0.0.0.0:3000")?
    .run()
}
