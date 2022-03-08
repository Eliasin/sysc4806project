#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_dyn_templates::Template;
use sass_rocket_fairing::SassFairing;
use rocket_dyn_templates::tera::Context;
use db::DbConn;

pub mod db;
pub mod models;
pub mod rest;
pub mod schema;


#[get("/")]
fn index() -> Template {
    let context = Context::new();
    Template::render("index", &context.into_json())
}

#[get("/prof")]
fn prof() -> Template {
    let context = Context::new();
    Template::render("prof-select", &context.into_json())
}

#[get("/student")]
fn student() -> Template {
    let context = Context::new();
    Template::render("student-select", &context.into_json())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", rest::routes())
        .mount("/", routes![index, prof, student])
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .attach(SassFairing)
        // .mount("/", routes![index])
        // .mount("/styles", FileServer::from(relative!("templates/styles")))
}
