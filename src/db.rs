use crate::models::*;
use crate::request_guards::state::SessionType;
use crate::rest::Login;
use crate::schema;
use anyhow::anyhow;
use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};
use serde::Serialize;

#[database("db")]
pub struct DbConn(diesel::PgConnection);

pub type ID = i32;

/// This function takes in a name of a reasearch field that can be converted to a string that is then
/// adds it to the database after generating a ResearchField ID.
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

/// This function returns every research field in the database
pub async fn get_research_fields(conn: &DbConn) -> QueryResult<Vec<ResearchField>> {
    use schema::research_fields::dsl::*;

    conn.run(|c| research_fields.select((id, name)).load::<ResearchField>(c))
        .await
}

/// This function takes in a ID of a reasearch field
/// that is then used to locate a specific research fild in the database and return it.
pub async fn get_research_field(
    conn: &DbConn,
    research_field_id: ID,
) -> QueryResult<Option<ResearchField>> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| research_fields.find(research_field_id).first(c).optional())
        .await
}

/// This function takes in a ID of a reasearch field
/// that is then used to locate a specific research fild in the database and delete it.
pub async fn delete_research_field(conn: &DbConn, research_field_id: ID) -> QueryResult<()> {
    use schema::research_fields::dsl::*;

    conn.run(move |c| diesel::delete(research_fields.find(research_field_id)).execute(c))
        .await?;
    Ok(())
}

/// This function takes in a name of a professor that can be converted to a string that is then
/// adds it to the database after generating a professor ID.
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

/// This function takes in a ID of a professor
/// that is then used to locate a specific professor in the database and return it.
pub async fn get_professor(conn: &DbConn, professor_id: ID) -> QueryResult<Option<Professor>> {
    use schema::professors::dsl::*;

    conn.run(move |c| professors.find(professor_id).first(c).optional())
        .await
}

// This function returns every professor in the database
pub async fn get_professors(conn: &DbConn) -> QueryResult<Vec<Professor>> {
    use schema::professors::dsl::*;

    conn.run(|c| professors.select((id, name)).load::<Professor>(c))
        .await
}

/// This function takes in a ID of a professor
/// that is then used to locate a specific professor in the database and return it.
pub async fn delete_professor(conn: &DbConn, professor_id: ID) -> QueryResult<()> {
    use schema::professors::dsl::*;

    conn.run(move |c| diesel::delete(professors.find(professor_id)).execute(c))
        .await?;
    Ok(())
}

/// This function takes in a ID of a professor and the ID of a research field
/// which are used to assaign a research field to a professor and adds both ids to a new table
/// in the database.
pub async fn add_researched_field_to_professor(
    conn: &DbConn,
    professor_id: ID,
    research_field_id: ID,
) -> QueryResult<()> {
    use schema::professor_research_fields;

    let new_professor_research_field = ProfessorResearchField {
        prof_id: professor_id.to_owned(),
        field_id: research_field_id.to_owned(),
    };

    conn.run(move |c| {
        diesel::insert_into(professor_research_fields::table)
            .values(&new_professor_research_field)
            .execute(c)
    })
    .await?;
    Ok(())
}

/// This function takes in a ID of a professor
/// which is then used to get all research fields linked to that professors ID
pub async fn get_fields_professor_researches(
    conn: &DbConn,
    professor_id: ID,
) -> QueryResult<Vec<ResearchField>> {
    use dsl_professor_research_fields::{prof_id, professor_research_fields};
    use dsl_research_fields::{id, name, research_fields};
    use schema::professor_research_fields::dsl as dsl_professor_research_fields;
    use schema::research_fields::dsl as dsl_research_fields;

    conn.run(move |c| {
        professor_research_fields
            .filter(prof_id.eq(professor_id))
            .inner_join(research_fields.on(id.eq(dsl_professor_research_fields::field_id)))
            .select((id, name))
            .load::<ResearchField>(c)
    })
    .await
}

/// This function takes in a ID of a professor and the ID of a research field
/// which are used to find the row in the table that links the professor to that research field
/// and then deletes it from the table
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

/// This function takes in an applicant which is then inserted into the applicant
/// table in the database
pub async fn create_applicant(conn: &DbConn, applicant: NewApplicant) -> QueryResult<ID> {
    use schema::applicants;

    conn.run(move |c| {
        diesel::insert_into(applicants::table)
            .values(&applicant)
            .returning(applicants::id)
            .get_result(c)
    })
    .await
}

/// This function takes in applicant information which is then to and applicant in the applicant
/// table in the database
pub async fn edit_applicant(conn: &DbConn, app_id: ID, app_data: ApplicantEdit) -> QueryResult<()> {
    use schema::applicants::dsl::*;

    if let Some(v) = app_data.name {
        conn.run(move |c| {
            diesel::update(applicants.find(app_id))
                .set(name.eq(v))
                .execute(c)
        })
        .await?;
    }
    if let Some(v) = app_data.email {
        conn.run(move |c| {
            diesel::update(applicants.find(app_id))
                .set(email.eq(v))
                .execute(c)
        })
        .await?;
    }
    if let Some(v) = app_data.phone_number {
        conn.run(move |c| {
            diesel::update(applicants.find(app_id))
                .set(phone_number.eq(v))
                .execute(c)
        })
        .await?;
    }
    if let Some(v) = app_data.desired_field_id {
        conn.run(move |c| {
            diesel::update(applicants.find(app_id))
                .set(desired_field_id.eq(v))
                .execute(c)
        })
        .await?;
    }

    Ok(())
}

pub async fn edit_professor(
    conn: &DbConn,
    prof_id: ID,
    prof_data: ProfessorEdit,
) -> QueryResult<()> {
    use schema::professors::dsl::*;

    conn.run(move |c| {
        diesel::update(professors.find(prof_id))
            .set(name.eq(prof_data.name))
            .execute(c)
    })
    .await?;

    Ok(())
}

/// This function takes in an applicant ID which is then used to find the applicant in the
/// database and return the applicant
pub async fn get_applicant(conn: &DbConn, applicant_id: ID) -> QueryResult<Option<Applicant>> {
    use schema::applicants::dsl::*;

    conn.run(move |c| applicants.find(applicant_id).first(c).optional())
        .await
}

// This function returns every applicant in the database
pub async fn get_applicants(conn: &DbConn) -> QueryResult<Vec<Applicant>> {
    use schema::applicants::dsl::*;

    conn.run(|c| applicants.load::<Applicant>(c)).await
}

/// This function takes in an applicant ID which is then used to find the applicant in the
/// database and delete the applicant
pub async fn delete_applicant(conn: &DbConn, applicant_id: ID) -> QueryResult<()> {
    use schema::applicants::dsl::*;

    conn.run(move |c| diesel::delete(applicants.find(applicant_id)).execute(c))
        .await?;
    Ok(())
}

pub const APPLICATION_ACCEPTED: &'static str = "ACCEPTED";
pub const APPLICATION_DENIED: &'static str = "DENIED";
pub const APPLICATION_PENDING: &'static str = "PENDING";

/// This function takes in an applicant ID and proffesor ID which are then added to a new table showing
/// specifiying that the applicant has applied to this professor.
pub async fn add_application_to_applicant(
    conn: &DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    use schema::student_applied_to;

    let new_student_applied_to = StudentAppliedTo {
        applicant_id: applicant_id.to_owned(),
        prof_id: professor_id.to_owned(),
        status: APPLICATION_PENDING.to_string(),
    };

    conn.run(move |c| {
        diesel::insert_into(student_applied_to::table)
            .values(&new_student_applied_to)
            .execute(c)
    })
    .await?;
    Ok(())
}

/// This function takes in a applicant ID and uses that to find all professors that the
/// applicant has applied to and returns them in a list.
pub async fn get_profs_applicant_applied_to(
    conn: &DbConn,
    applicant_id: ID,
) -> QueryResult<Vec<Professor>> {
    use dsl_professors::{id, name, professors};
    use dsl_student_applied_to::{applicant_id as dsl_applicant_id, prof_id, student_applied_to};
    use schema::professors::dsl as dsl_professors;
    use schema::student_applied_to::dsl as dsl_student_applied_to;

    conn.run(move |c| {
        student_applied_to
            .filter(dsl_applicant_id.eq(applicant_id))
            .inner_join(professors.on(id.eq(prof_id)))
            .select((id, name))
            .load::<Professor>(c)
    })
    .await
}

/// Sets an applicant's application status.
async fn set_applicant_application_status(
    conn: &DbConn,
    app_id: ID,
    professor_id: ID,
    new_status: String,
) -> QueryResult<()> {
    use schema::student_applied_to::dsl::*;

    conn.run(move |c| {
        diesel::update(student_applied_to.find((app_id, professor_id)))
            .set(status.eq(new_status))
            .execute(c)
    })
    .await?;

    Ok(())
}

/// Accepts an applicant's application.
pub async fn accept_applicant_application(
    conn: &DbConn,
    app_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    set_applicant_application_status(conn, app_id, professor_id, APPLICATION_ACCEPTED.to_string())
        .await
}

/// Denies an applicant's application.
pub async fn deny_applicant_application(
    conn: &DbConn,
    app_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    set_applicant_application_status(conn, app_id, professor_id, APPLICATION_DENIED.to_string())
        .await
}

/// Uploads a file blob for an applicant.
async fn upload_applicant_blob(conn: &DbConn, blob: Vec<u8>) -> QueryResult<ID> {
    use schema::applicant_blobs;

    conn.run(move |c| {
        diesel::insert_into(applicant_blobs::table)
            .values(NewApplicantBlob { data_blob: blob })
            .returning(schema::applicant_blobs::id)
            .get_result(c)
    })
    .await
}

/// Uploads a cv blob for an applicant.
pub async fn upload_applicant_cv(conn: &DbConn, applicant_id: ID, cv: Vec<u8>) -> QueryResult<()> {
    use schema::applicants::dsl::{applicants as applicants_dsl, cv_blob_id as app_cv_blob_id};

    let blob_id = upload_applicant_blob(conn, cv).await?;

    conn.run(move |c| {
        diesel::update(applicants_dsl.find(applicant_id))
            .set(app_cv_blob_id.eq(blob_id))
            .execute(c)
    })
    .await?;

    Ok(())
}

/// Uploads a diploma blob for an applicant.
pub async fn upload_applicant_diploma(
    conn: &DbConn,
    applicant_id: ID,
    diploma: Vec<u8>,
) -> QueryResult<()> {
    use schema::applicants::dsl::{
        applicants as applicants_dsl, diploma_blob_id as app_diploma_blob_id,
    };

    let blob_id = upload_applicant_blob(conn, diploma).await?;

    conn.run(move |c| {
        diesel::update(applicants_dsl.find(applicant_id))
            .set(app_diploma_blob_id.eq(blob_id))
            .execute(c)
    })
    .await?;

    Ok(())
}

/// Uploads a grade audit blob for an applicant.
pub async fn upload_applicant_grade_audit(
    conn: &DbConn,
    applicant_id: ID,
    grade_audit: Vec<u8>,
) -> QueryResult<()> {
    use schema::applicants::dsl::{
        applicants as applicants_dsl, grade_audit_blob_id as app_grade_audit_blob_id,
    };

    let blob_id = upload_applicant_blob(conn, grade_audit).await?;

    conn.run(move |c| {
        diesel::update(applicants_dsl.find(applicant_id))
            .set(app_grade_audit_blob_id.eq(blob_id))
            .execute(c)
    })
    .await?;

    Ok(())
}

/// Gets an applicant blob of data given its ID.
async fn get_applicant_blob(conn: &DbConn, blob_id: ID) -> QueryResult<Vec<u8>> {
    use schema::applicant_blobs::dsl::*;

    conn.run(move |c| applicant_blobs.find(blob_id).select(data_blob).first(c))
        .await
}

/// Gets an applicant's CV as blob of data.
pub async fn get_applicant_cv_blob(conn: &DbConn, applicant: Applicant) -> anyhow::Result<Vec<u8>> {
    if let Some(v) = applicant.cv_blob_id {
        Ok(get_applicant_blob(conn, v).await?)
    } else {
        Err(anyhow!("Applicant does not have a CV"))
    }
}

/// Gets an applicant's diploma as blob of data.
pub async fn get_applicant_diploma_blob(
    conn: &DbConn,
    applicant: Applicant,
) -> anyhow::Result<Vec<u8>> {
    if let Some(v) = applicant.diploma_blob_id {
        Ok(get_applicant_blob(conn, v).await?)
    } else {
        Err(anyhow!("Applicant does not have a diploma"))
    }
}

/// Gets an applicant's grade audit as blob of data.
pub async fn get_applicant_grade_audit_blob(
    conn: &DbConn,
    applicant: Applicant,
) -> anyhow::Result<Vec<u8>> {
    if let Some(v) = applicant.grade_audit_blob_id {
        Ok(get_applicant_blob(conn, v).await?)
    } else {
        Err(anyhow!("Applicant does not have a grade audit"))
    }
}

/// This function takes in an applicant ID and proffesor ID which are then used to find the
/// row in the table showing that they have applied to that professor and then deletes it.
pub async fn remove_application_from_applicant(
    conn: &DbConn,
    applicant_id: ID,
    professor_id: ID,
) -> QueryResult<()> {
    use schema::student_applied_to::dsl::student_applied_to;

    conn.run(move |c| {
        diesel::delete(student_applied_to.find((applicant_id, professor_id))).execute(c)
    })
    .await?;
    Ok(())
}

#[derive(Queryable, Serialize)]
pub struct ApplicantIDNameField {
    pub id: i32,
    pub desired_field: String,
    pub name: String,
    pub email: String,
}

pub async fn get_applications_for_professor_with_status(
    conn: &DbConn,
    professor_id: ID,
    status: String,
) -> QueryResult<Vec<ApplicantIDNameField>> {
    use schema::applicants::dsl::{
        applicants, desired_field_id as app_desired_field_id, email, id as app_id, name as app_name,
    };
    use schema::research_fields::dsl::{id as rs_id, name as rs_name, research_fields};
    use schema::student_applied_to::dsl::{
        applicant_id as sa_applicant_id, prof_id as sa_prof_id, status as sa_status,
        student_applied_to,
    };

    conn.run(move |c| {
        student_applied_to
            .filter(sa_prof_id.eq(professor_id))
            .filter(sa_status.eq(status))
            .inner_join(applicants.on(app_id.eq(sa_applicant_id)))
            .inner_join(research_fields.on(rs_id.eq(app_desired_field_id)))
            .select((app_id, rs_name, app_name, email))
            .load::<ApplicantIDNameField>(c)
    })
    .await
}

pub enum LoginError {
    CredentialError,
    DatabaseError,
}

#[derive(Queryable)]
pub struct UserIdHash {
    pub id: i32,
    pub password_hash: String,
}

pub async fn validate_login(
    conn: &DbConn,
    username: String,
    password: String,
) -> Result<SessionType, LoginError> {
    use schema::admin_logins::dsl::{
        admin_logins, bcrypt_hash as admin_password_hash, username as admin_username,
    };
    use schema::applicant_logins::dsl::{
        applicant_logins, bcrypt_hash as applicant_password_hash, id as db_applicant_id,
        username as applicant_username,
    };
    use schema::professor_logins::dsl::{
        bcrypt_hash as professor_password_hash, id as db_professor_id, professor_logins,
        username as professor_username,
    };

    let username = username.to_owned();
    let password = password.to_owned();
    {
        let username = username.clone();
        let password = password.clone();
        let applicant = conn
            .run(move |c| {
                applicant_logins
                    .filter(applicant_username.eq(username))
                    .select((db_applicant_id, applicant_password_hash))
                    .first::<UserIdHash>(c)
            })
            .await
            .optional()
            .map_err(|_| LoginError::DatabaseError)?;

        match applicant {
            Some(applicant) => {
                let applicant_id = applicant.id;
                let password_hash = applicant.password_hash;

                if bcrypt::verify(password, password_hash.as_str())
                    .map_err(|_| LoginError::CredentialError)?
                {
                    return Ok(SessionType::Applicant(applicant_id));
                }
            }
            None => {}
        };
    }

    {
        let username = username.clone();
        let password = password.clone();
        let professor = conn
            .run(move |c| {
                professor_logins
                    .filter(professor_username.eq(username))
                    .select((db_professor_id, professor_password_hash))
                    .first::<UserIdHash>(c)
            })
            .await
            .optional()
            .map_err(|_| LoginError::DatabaseError)?;

        match professor {
            Some(professor) => {
                let professor_id = professor.id;
                let password_hash = professor.password_hash;

                if bcrypt::verify(password, password_hash.as_str())
                    .map_err(|_| LoginError::CredentialError)?
                {
                    return Ok(SessionType::Professor(professor_id));
                }
            }
            None => {}
        };
    }

    let administrator = conn
        .run(move |c| {
            admin_logins
                .filter(admin_username.eq(username))
                .select(admin_password_hash)
                .first::<String>(c)
        })
        .await
        .optional()
        .map_err(|_| LoginError::DatabaseError)?;

    match administrator {
        Some(password_hash) => {
            if bcrypt::verify(password, password_hash.as_str())
                .map_err(|_| LoginError::CredentialError)?
            {
                return Ok(SessionType::Administrator);
            } else {
                return Err(LoginError::CredentialError);
            }
        }
        None => {}
    };

    Err(LoginError::CredentialError)
}

pub async fn create_admin_account(conn: &DbConn, login_data: Login) -> QueryResult<()> {
    use schema::admin_logins::dsl::admin_logins;

    let bcrypt_hash = bcrypt::hash(login_data.password.as_str(), bcrypt::DEFAULT_COST).unwrap();

    conn.run(move |c| {
        diesel::insert_into(admin_logins)
            .values(NewAdminLogin {
                username: login_data.username,
                bcrypt_hash,
            })
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn create_applicant_account(
    conn: &DbConn,
    applicant_id: i32,
    login_data: Login,
) -> QueryResult<()> {
    use schema::applicant_logins::dsl::applicant_logins;

    let bcrypt_hash = bcrypt::hash(login_data.password.as_str(), bcrypt::DEFAULT_COST).unwrap();

    conn.run(move |c| {
        diesel::insert_into(applicant_logins)
            .values(NewApplicantLogin {
                id: applicant_id,
                username: login_data.username,
                bcrypt_hash,
            })
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn create_professor_account(
    conn: &DbConn,
    professor_id: i32,
    login_data: Login,
) -> QueryResult<()> {
    use schema::professor_logins::dsl::professor_logins;

    let bcrypt_hash = bcrypt::hash(login_data.password.as_str(), bcrypt::DEFAULT_COST).unwrap();

    conn.run(move |c| {
        diesel::insert_into(professor_logins)
            .values(NewProfessorLogin {
                id: professor_id,
                username: login_data.username,
                bcrypt_hash,
            })
            .execute(c)
    })
    .await?;
    Ok(())
}

pub async fn admin_exists(conn: &DbConn) -> QueryResult<bool> {
    use schema::admin_logins::dsl::admin_logins;

    let admins_count = conn
        .run(move |c| admin_logins.count().get_result::<i64>(c))
        .await;

    match admins_count {
        Ok(count) => Ok(count > 0),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}
