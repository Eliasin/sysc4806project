#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_dyn_templates::Template;

pub mod models;
pub mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
