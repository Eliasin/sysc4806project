//! This module contains all of the domain object datatypes for the application.
//! Theese objects are defined in a datbase-centric way that makes them easy to
//! perform DB opertations with, but any clients of this module may want to define
//! specific datatypes as joins of these primitives for better ease of use.

use crate::schema::*;
use rocket::serde::{Deserialize, Serialize};

/// A research field defined by a name, research fields can share names as they
/// are uniquely identified with IDs.
#[derive(Queryable, Identifiable, PartialEq, Debug, Deserialize, Serialize)]
pub struct ResearchField {
    pub id: i32,
    pub name: String,
}

/// This type represents a request for a new research field. It does not include an ID
/// as they are auto-generated.
#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name = "research_fields"]
pub struct NewResearchField {
    pub name: String,
}

/// A professor defined by a name, professors can share names as they
/// are uniquely identified with IDs. Professors can advise multiple students
/// and research multiple fields. These relationships are encoded in
/// the professor_research_fields and student_applied_to tables.
#[derive(Queryable, Identifiable, PartialEq, Debug, Serialize)]
pub struct Professor {
    pub id: i32,
    pub name: String,
}

/// This type represents a request for a new professor. It does not include an ID
/// as they are auto-generated.
#[derive(Insertable, Debug, Deserialize)]
#[table_name = "professors"]
pub struct NewProfessor {
    pub name: String,
}

/// This type represents the relationship between a professor and a field that they
/// research. There are no extra attributes to this relationship.
#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Insertable)]
#[primary_key(prof_id, field_id)]
#[belongs_to(ResearchField, foreign_key = "field_id")]
#[belongs_to(Professor, foreign_key = "prof_id")]
pub struct ProfessorResearchField {
    pub prof_id: i32,
    pub field_id: i32,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
pub struct ApplicantBlob {
    pub id: i32,
    pub data_blob: Vec<u8>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "applicant_blobs"]
pub struct NewApplicantBlob {
    pub data_blob: Vec<u8>,
}

/// This type represents a graduate applicant and includes information about their
/// name, desired field, application details, etc. They are uniquely identified by ID.
#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(ResearchField, foreign_key = "desired_field_id")]
pub struct Applicant {
    pub id: i32,
    pub desired_field_id: i32,
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub cv_blob_id: Option<i32>,
    pub diploma_blob_id: Option<i32>,
    pub grade_audit_blob_id: Option<i32>,
}

/// This type represents a request for a new applicant. It does not include an ID
/// as they are auto-generated.
#[derive(Insertable, Deserialize)]
#[table_name = "applicants"]
pub struct NewApplicant {
    pub name: String,
    pub desired_field_id: i32,
    pub phone_number: String,
    pub email: String,
}

/// This type represents a request for editing an applicant.
#[derive(Insertable, Deserialize)]
#[table_name = "applicants"]
pub struct ApplicantEdit {
    pub name: Option<String>,
    pub desired_field_id: Option<i32>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

/// This type represents a request for editing an professor.
#[derive(Deserialize)]
pub struct ProfessorEdit {
    pub name: String,
}

/// This type represents the relationship between an applicant and a professor that they
/// applied to. There are no extra attributes to this relationship.
#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Insertable)]
#[primary_key(applicant_id, prof_id)]
#[belongs_to(Applicant, foreign_key = "applicant_id")]
#[belongs_to(Professor, foreign_key = "prof_id")]
#[table_name = "student_applied_to"]
pub struct StudentAppliedTo {
    pub applicant_id: i32,
    pub prof_id: i32,
    // Diesel does not have good support for Postgres enums, so we use strings for
    // the application status
    pub status: String,
}

/// This type represents a request for a new admin.
#[derive(Insertable)]
#[table_name = "admin_logins"]
pub struct NewAdminLogin {
    pub username: String,
    pub bcrypt_hash: String,
}

/// This type represents a request for a new applicant. It does not include an ID
/// as they are auto-generated.
#[derive(Insertable)]
#[table_name = "applicant_logins"]
pub struct NewApplicantLogin {
    pub username: String,
    pub bcrypt_hash: String,
}

/// This type represents a request for a new professor. It does not include an ID
/// as they are auto-generated.
#[derive(Insertable)]
#[table_name = "professor_logins"]
pub struct NewProfessorLogin {
    pub username: String,
    pub bcrypt_hash: String,
}
