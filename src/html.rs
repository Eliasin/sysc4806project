//! Servers html templates and static files from the `templates` and `static` directories.

use rocket::Route;
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

/// Endpoint for the home page.
#[get("/")]
fn index() -> Template {
    let context = Context::new();
    Template::render("index", &context.into_json())
}

/// Declaration of page navigation routes.
pub fn routes() -> Vec<Route> {
    routes![index]
}
