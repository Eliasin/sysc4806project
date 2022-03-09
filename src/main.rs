//! Establishes application instance.

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use db::DbConn;
use rocket_dyn_templates::Template;

pub mod db;
pub mod html;
pub mod models;
pub mod rest;
pub mod schema;

/// Builds the rocket instance with rest and html routes.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .mount("/", html::routes())
        .attach(Template::fairing())
        .attach(DbConn::fairing())
}
