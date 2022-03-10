//! Servers html templates and static files from the `templates` and `static` directories.

use crate::db;
use crate::db::DbConn;
use rocket::http::Status;
use rocket::Route;
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::Template;

pub type TemplateResult = Result<Template, Status>;

/// Endpoint for the home page.
#[get("/")]
pub async fn index(conn: DbConn) -> TemplateResult {
    let mut context = Context::new();

    let research_fields = match db::get_research_fields(&conn).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching research fields: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    let professors = match db::get_professors(&conn).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching professors: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    context.insert("research_fields", &research_fields);
    context.insert("professors", &professors);

    Ok(Template::render("index", &context.into_json()))
}

/// Declaration of page navigation routes.
pub fn routes() -> Vec<Route> {
    routes![index]
}
