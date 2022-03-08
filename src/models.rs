use crate::schema::*;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
pub struct ResearchField {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "research_fields"]
pub struct NewResearchField {
    pub name: String,
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
pub struct Professor {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(prof_id, field_id)]
#[belongs_to(ResearchField, foreign_key = "field_id")]
#[belongs_to(Professor, foreign_key = "prof_id")]
pub struct ProfessorResearchField {
    pub prof_id: i32,
    pub field_id: i32,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(ResearchField, foreign_key = "desired_field_id")]
pub struct Applicant {
    pub id: i32,
    pub name: String,
    pub desired_field_id: i32,
    pub phone_number: String,
    pub email: String,
    pub cv_path: String,
    pub diploma_path: String,
    pub grade_audit_path: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(applicant_id, prof_id)]
#[belongs_to(Applicant, foreign_key = "applicant_id")]
#[belongs_to(Professor, foreign_key = "prof_id")]
#[table_name = "student_applied_to"]
pub struct StudentAppliedTo {
    pub applicant_id: i32,
    pub prof_id: i32,
}
