//! Establishes application instance.

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use db::DbConn;
use rocket_dyn_templates::Template;
use rocket::fs::FileServer;

pub mod db;
pub mod html;
pub mod models;
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
            res.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, PATCH, OPTIONS, DELETE",
            ));
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

/// Builds the rocket instance with rest and html routes.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .mount("/", html::routes())
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .attach(CORS::fairing())
}
