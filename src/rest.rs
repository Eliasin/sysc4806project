//! Defines the REST endpoints for the Graduate Admissions Management System API.

use crate::db::{self, APPLICATION_ACCPETED, APPLICATION_DENIED, APPLICATION_PENDING, ID};
use crate::db::{ApplicantIDNameField, DbConn};
use crate::models::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use serde::{Deserialize, Serialize};

/// Type representing an id returned for newly created entities.
#[derive(Serialize, Deserialize, Debug)]
struct IdPayload {
    id: ID,
}

/// Endpoint for creating a new research field.
#[post("/research-field", data = "<research_field>")]
async fn create_research_field(
    conn: DbConn,
    research_field: Json<NewResearchField>,
) -> Result<Json<IdPayload>, Status> {
    match db::create_research_field(&conn, research_field.into_inner().name).await {
        Ok(id) => Ok(Json(IdPayload { id })),
        Err(e) => {
            eprintln!(
                "DB error occured while trying to create research field: {}",
                e
            );
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting a research field.
#[get("/research-field?<id>")]
async fn get_research_field(conn: DbConn, id: i32) -> Result<Json<ResearchField>, Status> {
    match db::get_research_field(&conn, id).await {
        Ok(research_field) => Ok(Json(research_field)),
        Err(e) => {
            eprintln!("DB error occured while trying to get research field: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting all research fields.
#[get("/research-fields")]
async fn get_research_fields(conn: DbConn) -> Result<Json<Vec<ResearchField>>, Status> {
    match db::get_research_fields(&conn).await {
        Ok(research_fields) => Ok(Json(research_fields)),
        Err(e) => {
            eprintln!("DB error occured while trying to get research field: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for deleting a research field.
#[delete("/research-field?<id>")]
async fn delete_research_field(conn: DbConn, id: i32) -> Status {
    match db::delete_research_field(&conn, id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!(
                "DB error occured while trying to delete research field: {}",
                e
            );
            Status::InternalServerError
        }
    }
}

/// Endpoint for creating a new professor.
#[post("/professor", data = "<professor>")]
async fn create_professor(
    conn: DbConn,
    professor: Json<NewProfessor>,
) -> Result<Json<IdPayload>, Status> {
    match db::create_professor(&conn, professor.into_inner().name).await {
        Ok(id) => Ok(Json(IdPayload { id })),
        Err(e) => {
            eprintln!("DB error occured while trying to create professor: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting a professor.
#[get("/professor?<id>")]
async fn get_professor(conn: DbConn, id: i32) -> Result<Json<Professor>, Status> {
    match db::get_professor(&conn, id).await {
        Ok(professor) => Ok(Json(professor)),
        Err(e) => {
            eprintln!("DB error occured while trying to get professor: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting all professors.
#[get("/professors")]
async fn get_professors(conn: DbConn) -> Result<Json<Vec<Professor>>, Status> {
    match db::get_professors(&conn).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!("DB error occured while trying to get professors: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for deleting a professor.
#[delete("/professor?<id>")]
async fn delete_professor(conn: DbConn, id: i32) -> Status {
    match db::delete_professor(&conn, id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!("DB error occured while trying to delete professor: {}", e);
            Status::InternalServerError
        }
    }
}

/// Endpoint for adding a research field to a professor.
#[post("/professor/research-field?<prof_id>&<field_id>")]
async fn add_researched_field_to_professor(conn: DbConn, prof_id: i32, field_id: i32) -> Status {
    match db::add_researched_field_to_professor(&conn, prof_id, field_id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!(
                "DB error occured while trying to add researched field to professor: {}",
                e
            );
            Status::InternalServerError
        }
    }
}

/// Endpoint for getting a list of research fields for a professor.
#[get("/professor/research-field?<prof_id>")]
async fn get_fields_professor_researches(
    conn: DbConn,
    prof_id: i32,
) -> Result<Json<Vec<ResearchField>>, Status> {
    match db::get_fields_professor_researches(&conn, prof_id).await {
        Ok(research_fields) => Ok(Json(research_fields)),
        Err(e) => {
            eprintln!(
                "DB error occured while trying to get fields professor researches: {}",
                e
            );
            Err(Status::InternalServerError)
        }
    }
}

#[get("/professor/applicants?<id>&<status>")]
async fn get_applicants_for_professor_with_status(
    conn: DbConn,
    id: i32,
    status: String,
) -> Result<Json<Vec<ApplicantIDNameField>>, Status> {
    match status.as_str() {
        APPLICATION_ACCPETED => {}
        APPLICATION_DENIED => {}
        APPLICATION_PENDING => {}
        _ => {
            eprintln!(
                "Client asked for bad status, no status known as: {}",
                status
            );
            return Err(Status::BadRequest);
        }
    };

    match db::get_applications_for_professor_with_status(&conn, id, status.clone()).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!(
                "Error while retrieving applicants for professor with status {}: {}",
                status, e
            );
            return Err(Status::InternalServerError);
        }
    }
}

/// Endpoint for removing a research field from a professor.
#[delete("/professor/research-field?<prof_id>&<field_id>")]
async fn remove_researched_field_from_professor(
    conn: DbConn,
    prof_id: i32,
    field_id: i32,
) -> Status {
    match db::remove_researched_field_from_professor(&conn, prof_id, field_id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!(
                "DB error occured while trying to add researched field to professor: {}",
                e
            );
            Status::InternalServerError
        }
    }
}

#[post("/professor/application/accept?<applicant_id>&<professor_id>")]
pub async fn accept_application(
    conn: DbConn,
    applicant_id: i32,
    professor_id: i32,
) -> Result<(), Status> {
    if let Err(e) = db::accept_applicant_application(&conn, applicant_id, professor_id).await {
        eprintln!("Error while accepting an applicant's application: {}", e);
        Err(Status::InternalServerError)
    } else {
        Ok(())
    }
}

#[post("/professor/application/deny?<applicant_id>&<professor_id>")]
pub async fn deny_application(
    conn: DbConn,
    applicant_id: i32,
    professor_id: i32,
) -> Result<(), Status> {
    if let Err(e) = db::deny_applicant_application(&conn, applicant_id, professor_id).await {
        eprintln!("Error while denying an applicant's application: {}", e);
        Err(Status::InternalServerError)
    } else {
        Ok(())
    }
}

/// Endpoint for creating a new applicant.
#[post("/applicant", data = "<applicant>")]
async fn create_applicant(
    conn: DbConn,
    applicant: Json<NewApplicant>,
) -> Result<Json<IdPayload>, Status> {
    match db::create_applicant(&conn, applicant.into_inner()).await {
        Ok(id) => Ok(Json(IdPayload { id })),
        Err(e) => {
            eprintln!("DB error occured while trying to create applicant: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting an applicant.
#[get("/applicant?<id>")]
async fn get_applicant(conn: DbConn, id: i32) -> Result<Json<Applicant>, Status> {
    match db::get_applicant(&conn, id).await {
        Ok(applicant) => Ok(Json(applicant)),
        Err(e) => {
            eprintln!("DB error occured while trying to get applicant: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for getting all applicants.
#[get("/applicants")]
async fn get_applicants(conn: DbConn) -> Result<Json<Vec<Applicant>>, Status> {
    match db::get_applicants(&conn).await {
        Ok(applicants) => Ok(Json(applicants)),
        Err(e) => {
            eprintln!("DB error occured while trying to get research field: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for deleting an applicant.
#[delete("/applicant?<id>")]
async fn delete_applicant(conn: DbConn, id: i32) -> Status {
    match db::delete_applicant(&conn, id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!("DB error occured while trying to delete applicant: {}", e);
            Status::InternalServerError
        }
    }
}

/// Endpoint for adding an application to an applicant.
#[post("/applicant/applications?<applicant_id>&<prof_id>")]
async fn add_application_to_applicant(conn: DbConn, applicant_id: i32, prof_id: i32) -> Status {
    match db::add_application_to_applicant(&conn, applicant_id, prof_id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!(
                "DB error occured while trying to add application to applicant: {}",
                e
            );
            Status::InternalServerError
        }
    }
}

#[derive(FromForm)]
pub struct ApplicantFiles {
    cv_file: Option<Vec<u8>>,
    diploma_file: Option<Vec<u8>>,
    grade_audit_file: Option<Vec<u8>>,
}

/// Endpoint for adding a cv file to an applicant.
#[post("/applicant/files?<applicant_id>", data = "<files>")]
async fn upload_applicant_files(
    conn: DbConn,
    applicant_id: i32,
    files: Form<ApplicantFiles>,
) -> Result<(), Status> {
    if let Some(cv_file) = &files.cv_file {
        if let Err(e) = db::upload_applicant_cv(&conn, applicant_id, cv_file.clone()).await {
            eprintln!(
                "DB error occured while trying to upload applicant cv: {}",
                e
            );

            return Err(Status::InternalServerError);
        }
    }

    if let Some(diploma_file) = &files.diploma_file {
        if let Err(e) =
            db::upload_applicant_diploma(&conn, applicant_id, diploma_file.clone()).await
        {
            eprintln!(
                "DB error occured while trying to upload applicant diploma: {}",
                e
            );

            return Err(Status::InternalServerError);
        }
    }

    if let Some(grade_audit_file) = &files.grade_audit_file {
        if let Err(e) =
            db::upload_applicant_grade_audit(&conn, applicant_id, grade_audit_file.clone()).await
        {
            eprintln!(
                "DB error occured while trying to upload applicant grade audit: {}",
                e
            );

            return Err(Status::InternalServerError);
        }
    }

    Ok(())
}

#[get("/applicant/files/cv?<applicant_id>")]
async fn get_applicant_cv(conn: DbConn, applicant_id: i32) -> Result<Vec<u8>, Status> {
    let applicant = match db::get_applicant(&conn, applicant_id).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching applicant: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    match db::get_applicant_cv_blob(&conn, applicant).await {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("DB error while getting applicant cv blob: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/applicant/files/diploma?<applicant_id>")]
async fn get_applicant_diploma(conn: DbConn, applicant_id: i32) -> Result<Vec<u8>, Status> {
    let applicant = match db::get_applicant(&conn, applicant_id).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching applicant: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    match db::get_applicant_diploma_blob(&conn, applicant).await {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("DB error while getting applicant cv blob: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/applicant/files/grade-audit?<applicant_id>")]
async fn get_applicant_grade_audit(conn: DbConn, applicant_id: i32) -> Result<Vec<u8>, Status> {
    let applicant = match db::get_applicant(&conn, applicant_id).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("DB error while fetching applicant: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    match db::get_applicant_grade_audit_blob(&conn, applicant).await {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("DB error while getting applicant cv blob: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}

/// Endpoint for getting a list of professors an applicant has applied to.
#[get("/applicant/applications?<applicant_id>")]
async fn get_profs_applicant_applied_to(
    conn: DbConn,
    applicant_id: i32,
) -> Result<Json<Vec<Professor>>, Status> {
    match db::get_profs_applicant_applied_to(&conn, applicant_id).await {
        Ok(professors) => Ok(Json(professors)),
        Err(e) => {
            eprintln!(
                "DB error occured while trying to get profs applicant applied to: {}",
                e
            );
            Err(Status::InternalServerError)
        }
    }
}

/// Endpoint for removing an application from an applicant.
#[delete("/applicant/applications?<applicant_id>&<prof_id>")]
async fn remove_application_from_applicant(
    conn: DbConn,
    applicant_id: i32,
    prof_id: i32,
) -> Status {
    match db::remove_application_from_applicant(&conn, applicant_id, prof_id).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            eprintln!(
                "DB error occured while trying to remove application from applicant: {}",
                e
            );
            Status::InternalServerError
        }
    }
}

/// Declaration of REST request routes.
pub fn routes() -> Vec<Route> {
    routes![
        create_research_field,
        get_research_field,
        get_research_fields,
        delete_research_field,
        create_professor,
        get_professor,
        delete_professor,
        add_researched_field_to_professor,
        get_fields_professor_researches,
        remove_researched_field_from_professor,
        create_applicant,
        get_applicant,
        get_applicants,
        delete_applicant,
        add_application_to_applicant,
        get_profs_applicant_applied_to,
        remove_application_from_applicant,
        upload_applicant_files,
        get_applicant_cv,
        get_applicant_diploma,
        get_applicant_grade_audit,
        get_applicants_for_professor_with_status,
        get_professors,
    ]
}

/// REST endpoint tests.
#[cfg(test)]
mod test {
    use std::env::set_var;

    use crate::{
        models::{NewResearchField, ResearchField},
        rest::IdPayload,
        rocket,
    };
    use diesel::RunQueryDsl;
    use rocket::{
        http::Status,
        local::asynchronous::{Client, LocalResponse},
        serde::DeserializeOwned,
    };

    use super::DbConn;

    // Checks whether the current database is a testing database.
    fn is_testing_db<T: AsRef<str>>(url: T) -> bool {
        let lowercase = url.as_ref().to_ascii_lowercase();
        lowercase.contains("testing") || lowercase.contains("test")
    }

    // This resource is only accessible during testing, as only test setup functions mount it
    #[delete("/delete")]
    async fn reset_database(conn: DbConn) {
        conn.run(|c| {
            use crate::schema;
            use schema::applicants::dsl::*;
            use schema::professor_research_fields::dsl::*;
            use schema::professors::dsl::*;
            use schema::research_fields::dsl::*;
            use schema::student_applied_to::dsl::*;

            diesel::delete(student_applied_to)
                .execute(c)
                .expect("could not delete student_applied_to table");
            diesel::delete(professor_research_fields)
                .execute(c)
                .expect("could not delete professor_research_fields table");
            diesel::delete(applicants)
                .execute(c)
                .expect("could not delete applicants table");
            diesel::delete(professors)
                .execute(c)
                .expect("could not delete professors table");
            diesel::delete(research_fields)
                .execute(c)
                .expect("could not delete research_fields table");
        })
        .await;
    }

    // Configures and connects the dabatase for testing.
    const DATABASE_URL_KEY: &'static str = "databases.db.url";
    async fn setup() -> Client {
        set_var("ROCKET_PROFILE", "testing");

        let rocket = rocket();
        let db_url = rocket
            .figment()
            .find_value(DATABASE_URL_KEY)
            .expect("no database url configured")
            .into_string()
            .expect("database url cannot be converted to string");

        if !is_testing_db(&db_url) {
            panic!(
                "DB url {} is not a testing database url (must contain test or testing)",
                db_url
            );
        }

        let client = Client::tracked(rocket.mount("/testing-only", routes![reset_database]))
            .await
            .expect("valid rocket instance");

        client.delete("/testing-only/delete").dispatch().await;

        client
    }

    // The into_json method of the async LocalResponse tends to hangs as of v0.5-rc1 (https://github.com/SergioBenitez/Rocket/issues/1893)
    async fn to_json_workaround<T: DeserializeOwned>(response: LocalResponse<'_>) -> T {
        let body = response
            .into_string()
            .await
            .expect("response body could not be converted to string");

        serde_json::from_str(&body).expect("could not deserialize response body into JSON")
    }

    // Tests the creation of a research field.
    #[rocket::async_test]
    async fn create_get_research_field() {
        let client = setup().await;

        let biology = NewResearchField {
            name: "Biology".to_string(),
        };

        println!("Sending first create message...");
        let create_response = client
            .post("/rest/research-field")
            .json(&biology)
            .dispatch()
            .await;

        assert_eq!(create_response.status(), Status::Ok);
        let id = to_json_workaround::<IdPayload>(create_response).await.id;

        let biology_get_response = client
            .get(format!("/rest/research-field?id={}", id))
            .dispatch()
            .await;

        assert_eq!(biology_get_response.status(), Status::Ok);
        assert_eq!(
            to_json_workaround::<ResearchField>(biology_get_response).await,
            ResearchField {
                id,
                name: biology.name
            }
        );
    }
}
