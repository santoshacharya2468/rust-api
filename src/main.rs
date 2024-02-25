use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use routes::auth_routes;
use std::cell::RefCell;
pub mod controllers;
mod models;
mod schema;
mod services;
mod routes;
mod dtos;
pub struct AppState {
    pub db_con: RefCell<PgConnection>,
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
       App::new().app_data(web::Data::new(AppState {
            db_con: RefCell::new(
                diesel::PgConnection::establish(
                    "postgres://postgres:manakamana@localhost:5432/rustdb",
                )
                .unwrap(),
            ),
        }))
        .configure(auth_routes::routes) 
    });
    server.bind("127.0.0.1:8082").unwrap().run().await.unwrap();
}
