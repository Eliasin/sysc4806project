use rocket::Route;
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

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

pub fn routes() -> Vec<Route> {
    routes![index, prof, student]
}
