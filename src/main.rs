extern crate failure;

use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use r2d2::PooledConnection;
use actix_web::{Error};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use futures::{future::ok, Future};
use std::sync::Mutex;


mod communication;

mod auth;
use auth::auth_routes;

mod commontypes;



fn connect_pg() -> PooledConnection<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new("postgres://postgres:postgres@localhost:5432/actix-web",
                                                 TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    let connection = pool.get().unwrap();
    return connection;
}

fn index_async(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap())))
}




fn main() -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(connect_pg()));
    HttpServer::new(move ||
        App::new()
            .register_data(state.clone())
            .service(auth_routes())
            .service(web::resource("/hello/{name}").route(web::get().to_async(index_async)))
        )
        .bind("127.0.0.1:8080")?
        .run()
}
