use crate::models::*;
use crate::schema;
use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

#[database("db")]
pub struct DbConn(diesel::PgConnection);

pub type ID = i32;

pub async fn create_research_field<T: AsRef<str>>(conn: DbConn, name: T) -> QueryResult<ID> {
    use schema::research_fields;

    let new_research_field = NewResearchField {
        name: name.as_ref().to_owned(),
    };

    conn.run(move |c| {
        diesel::insert_into(research_fields::table)
            .values(&new_research_field)
            .returning(research_fields::id)
            .get_result(c)
    })
    .await
}

pub async fn get_research_field(conn: DbConn, research_field_id: ID) -> QueryResult<ResearchField> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| research_fields.find(research_field_id).first(c))
        .await
}

pub async fn delete_research_field(conn: DbConn, research_field_id: ID) -> QueryResult<()> {
    unimplemented!()
}

pub async fn create_professor<T: AsRef<str>>(conn: DbConn, name: T) -> QueryResult<ID> {
    unimplemented!()
}

pub async fn get_professor(conn: DbConn, professor_id: ID) -> QueryResult<Professor> {
    unimplemented!()
}

pub async fn delete_professor(conn: DbConn, professor_id: ID) -> QueryResult<()> {
    unimplemented!()
}

pub async fn add_researched_field_to_professor(
    conn: DbConn,
    professor_id: ID,
    research_field_id: ID,
) -> QueryResult<()> {
    unimplemented!()
}

pub async fn get_fields_professor_researches(
    conn: DbConn,
    professor_id: ID,
) -> QueryResult<Vec<ResearchField>> {
    unimplemented!()
}

pub async fn remove_researched_field_from_professor(
    conn: DbConn,
    professor_id: ID,
    research_field_id: ID,
) -> QueryResult<()> {
    unimplemented!()
}

pub async fn create_applicant<T: AsRef<str>>(
    conn: DbConn,
    applicant: NewApplicant,
) -> QueryResult<ID> {
    unimplemented!()
}

pub async fn get_applicant(conn: DbConn, applicant_id: ID) -> QueryResult<Applicant> {
    unimplemented!()
}

pub async fn delete_applicant(conn: DbConn, applicant_id: ID) -> QueryResult<()> {
    unimplemented!()
}

pub async fn add_application_to_applicant(
    conn: DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    unimplemented!()
}

pub async fn get_profs_applicant_applied_to(
    conn: DbConn,
    applicant_id: ID,
) -> QueryResult<Vec<Professor>> {
    unimplemented!()
}

pub async fn remove_application_from_applicant(
    conn: DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    unimplemented!()
}
