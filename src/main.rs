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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .mount("/", html::routes())
        .attach(Template::fairing())
        .attach(DbConn::fairing())
    // .mount("/", routes![index])
    // .mount("/styles", FileServer::from(relative!("templates/styles")))
}
