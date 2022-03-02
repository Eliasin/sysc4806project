use rocket_dyn_templates::{tera::Context, Template};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Buddy {
    name: String,
    number: String,
}

impl Buddy {
    pub fn new(name: String, number: String) -> Buddy {
        Buddy { name, number }
    }
}

#[get("/show")]
fn show() -> Template {
    let mut context = Context::new();

    context.insert(
        "buddy",
        &Buddy::new("Kaue".to_string(), "12312412".to_string()),
    );

    Template::render("show", context.into_json())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, show])
        .attach(Template::fairing())
}
