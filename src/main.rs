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

#[get("/", format = "text/html")]
pub async fn index() -> &'static str {
    "Hello World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    /*
    .mount("/rest", rest::routes())
    .mount("/", html::routes())
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    */
    // .mount("/", routes![index])
    // .mount("/styles", FileServer::from(relative!("templates/styles")))
}
