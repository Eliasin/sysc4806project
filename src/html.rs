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

/// Endpoint for applicant management.
#[get("/applicants")]
pub async fn applicants(_conn: DbConn) -> TemplateResult {
    let context = Context::new();

    Ok(Template::render("applicant_select", &context.into_json()))
}

#[get("/applicants/new")]
pub async fn applicant_create(conn: DbConn) -> TemplateResult {
    let mut context = Context::new();

    let applicant_list = match db::get_applicants(&conn).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching research fields: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    context.insert("applicants", &applicant_list);

    let mut research_fields = match db::get_research_fields(&conn).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching research fields: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    while research_fields.len() < 1 {
        match db::create_research_field(&conn, "EMPTY").await {
            Ok(_) => {
                research_fields = match db::get_research_fields(&conn).await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("DB error while fetching research fields: {}", e);
                        return Err(Status::InternalServerError);
                    }
                };
            }
            Err(e) => {
                eprintln!("DB error while creating research field: {}", e);
                return Err(Status::InternalServerError);
            }
        };
    }

    context.insert("research_fields", &research_fields);

    Ok(Template::render("applicant_create", &context.into_json()))
}

#[get("/applicants/list")]
pub async fn applicant_list(conn: DbConn) -> TemplateResult {
    let mut context = Context::new();

    let applicant_list = match db::get_applicants(&conn).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching research fields: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    context.insert("applicants", &applicant_list);

    Ok(Template::render("applicant_list", &context.into_json()))
}

/// Declaration of page navigation routes.
pub fn routes() -> Vec<Route> {
    routes![index, applicants, applicant_create, applicant_list]
}
