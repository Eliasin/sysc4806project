use crate::models::*;
use crate::schema;
use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

#[database("db")]
pub struct DbConn(diesel::PgConnection);

pub type ID = i32;

pub async fn create_research_field<T: AsRef<str>>(conn: &DbConn, name: T) -> QueryResult<ID> {
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

pub async fn get_research_field(
    conn: &DbConn,
    research_field_id: ID,
) -> QueryResult<ResearchField> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| research_fields.find(research_field_id).first(c))
        .await
}

pub async fn delete_research_field(conn: &DbConn, research_field_id: ID) -> QueryResult<()> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| {
        diesel::delete(research_fields.find(research_field_id)).execute(c)
    })
    .await?;
    Ok(())
}

pub async fn create_professor<T: AsRef<str>>(conn: &DbConn, name: T) -> QueryResult<ID> {
    use schema::professors;

    let new_professor = NewProfessor {
        name: name.as_ref().to_owned(),
    };

    conn.run(move |c| {
        diesel::insert_into(professors::table)
            .values(&new_professor)
            .returning(professors::id)
            .get_result(c)
    })
    .await
}

pub async fn get_professor(conn: &DbConn, professor_id: ID) -> QueryResult<Professor> {
    use schema::professors::dsl::*;

    conn.run(move |c| professors.find(professor_id).first(c))
        .await
}

pub async fn delete_professor(conn: &DbConn, professor_id: ID) -> QueryResult<()> {
    use schema::professors::dsl::*;

    conn.run(move |c| {
        diesel::delete(professors.find(professor_id)).execute(c)
    })
    .await?;
    Ok(())
}

pub async fn add_researched_field_to_professor(
    conn: &DbConn,
    professor_id: ID,
    research_field_id: ID,
) -> QueryResult<()> {
    use schema::professor_research_fields;

    let new_professor_research_field = ProfessorResearchField {
        prof_id: professor_id.to_owned(),
        field_id: research_field_id.to_owned()
    };

    conn.run(move |c| {
        diesel::insert_into(professor_research_fields::table)
            .values(&new_professor_research_field)
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn get_fields_professor_researches(
    conn: &DbConn,
    professor_id: ID,
) -> QueryResult<Vec<ResearchField>> {
    use schema::professor_research_fields::dsl as dsl_professor_research_fields;
    use dsl_professor_research_fields::{prof_id, professor_research_fields};
    use schema::research_fields::dsl as dsl_research_fields;
    use dsl_research_fields::{id, name, research_fields};

    conn.run(move |c| {
        professor_research_fields.filter(prof_id.eq(professor_id))
        .inner_join(research_fields.on(id.eq(prof_id))).select((id, name))
        .load::<ResearchField>(c)
    })
    .await
}

pub async fn remove_researched_field_from_professor(
    conn: &DbConn,
    professor_id: ID,
    research_field_id: ID,
) -> QueryResult<()> {
    use schema::professor_research_fields::dsl::*;

    conn.run(move |c| {
        diesel::delete(professor_research_fields.find((professor_id, research_field_id))).execute(c)
    })
    .await?;
    Ok(())
}

pub async fn create_applicant(
    conn: &DbConn,
    applicant: NewApplicant,
) -> QueryResult<ID> {
    use schema::applicants;

    conn.run(move |c| {
        diesel::insert_into(applicants::table)
            .values(&applicant)
            .returning(applicants::id)
            .get_result(c)
    })
    .await
}

pub async fn get_applicant(conn: &DbConn, applicant_id: ID) -> QueryResult<Applicant> {
    use schema::applicants::dsl::*;

    conn.run(move |c| applicants.find(applicant_id).first(c))
        .await
}

pub async fn delete_applicant(conn: &DbConn, applicant_id: ID) -> QueryResult<()> {
    use schema::applicants::dsl::*;

    conn.run(move |c| {
        diesel::delete(applicants.find(applicant_id)).execute(c)
    })
    .await?;
    Ok(())
}

pub async fn add_application_to_applicant(
    conn: &DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    use schema::student_applied_to;

    let new_student_applied_to = StudentAppliedTo {
        applicant_id: applicant_id.to_owned(),
        prof_id: professor_id.to_owned()
    };

    conn.run(move |c| {
        diesel::insert_into(student_applied_to::table)
            .values(&new_student_applied_to)
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn get_profs_applicant_applied_to(
    conn: &DbConn,
    applicant_id: ID,
) -> QueryResult<Vec<Professor>> {
    use schema::student_applied_to::dsl as dsl_student_applied_to;
    use dsl_student_applied_to::{applicant_id as dsl_applicant_id, prof_id, student_applied_to};
    use schema::professors::dsl as dsl_professors;
    use dsl_professors::{id, name, professors};

    conn.run(move |c| {
        student_applied_to.filter(dsl_applicant_id.eq(applicant_id))
        .inner_join(professors.on(id.eq(prof_id))).select((id, name))
        .load::<Professor>(c)
    })
    .await
}

pub async fn remove_application_from_applicant(
    conn: &DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    use schema::student_applied_to::dsl::{student_applied_to};

    conn.run(move |c| {
        diesel::delete(student_applied_to.find((applicant_id, professor_id))).execute(c)
    })
    .await?;
    Ok(())
}
