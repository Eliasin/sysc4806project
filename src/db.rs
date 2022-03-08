use rocket_sync_db_pools::{diesel, database};
use crate::models::*;
use crate::schema;
use diesel::prelude::*;

#[database("db")]
pub struct DbConn(diesel::PgConnection);

pub async fn create_research_field<T: AsRef<str>>(conn: DbConn, name: T) -> diesel::result::QueryResult<i32> {
    use schema::research_fields;

    let new_research_field = NewResearchField {
        name: name.as_ref().to_owned(),
    };

    conn.run(move |c| {
        diesel::insert_into(research_fields::table)
        .values(&new_research_field)
        .returning(research_fields::id)
        .get_result(c)
    }).await
}

pub async fn get_research_field(conn: DbConn, research_field_id: i32) -> QueryResult<ResearchField> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| {
        research_fields.find(research_field_id).first(c)
    }).await
    
}