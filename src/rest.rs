use crate::db::DbConn;
use crate::db::{self, ID};
use crate::models::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use serde::Serialize;

#[derive(Serialize)]
struct IdPayload {
    id: ID,
}

#[post("/research-field", data = "<research_field>")]
async fn create_research_field(
    conn: DbConn,
    research_field: Json<NewResearchField>,
) -> Result<Json<IdPayload>, Status> {
    match db::create_research_field(&conn, research_field.into_inner().name).await {
        Ok(id) => Ok(Json(IdPayload { id })),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/research-field?<id>")]
async fn get_research_field(conn: DbConn, id: i32) -> Result<Json<ResearchField>, Status> {
    match db::get_research_field(&conn, id).await {
        Ok(research_field) => Ok(Json(research_field)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn routes() -> Vec<Route> {
    routes![create_research_field, get_research_field]
}
