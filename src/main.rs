//! Establishes application instance.
#![feature(try_blocks)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use db::DbConn;
use request_guards::state::SessionTokens;
use rocket::futures::lock::Mutex;
use std::sync::Arc;

pub type SessionTokenState = Arc<Mutex<SessionTokens>>;

pub mod db;
pub mod models;
pub mod request_guards;
pub mod rest;
pub mod schema;

mod fairings {
    use rocket::{
        fairing::{Fairing, Info, Kind},
        http::Header,
    };

    pub struct CORS {}

    #[rocket::async_trait]
    impl Fairing for CORS {
        fn info(&self) -> Info {
            Info {
                name: "Adds CORS headers to responses",
                kind: Kind::Response,
            }
        }

        async fn on_response<'r>(
            &self,
            _req: &'r rocket::Request<'_>,
            res: &mut rocket::Response<'r>,
        ) {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res.set_header(Header::new("Access-Control-Allow-Methods", "*"));
            res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
    }

    impl CORS {
        pub fn fairing() -> CORS {
            CORS {}
        }
    }
}

use fairings::CORS;

#[options("/<_..>")]
pub async fn wildcard_options() {}

/// Builds the rocket instance with rest and html routes.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .mount("/rest", routes![wildcard_options])
        .manage(SessionTokenState::new(Mutex::new(SessionTokens::new())))
        .attach(DbConn::fairing())
        .attach(CORS::fairing())
}
