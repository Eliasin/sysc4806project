#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_dyn_templates::Template;

pub mod db;
pub mod models;
pub mod rest;
pub mod schema;

use db::DbConn;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .attach(Template::fairing())
        .attach(DbConn::fairing())
}
