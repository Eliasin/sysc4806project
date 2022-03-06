use rocket_dyn_templates::{tera::Context, Template};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, show])
        .attach(Template::fairing())
}
