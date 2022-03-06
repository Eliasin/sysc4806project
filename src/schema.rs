table! {
    applicants (id) {
        id -> Int4,
        desired_field_id -> Nullable<Int4>,
        name -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        email -> Nullable<Text>,
        cv_path -> Nullable<Text>,
        diploma_path -> Nullable<Text>,
        grade_audit_path -> Nullable<Text>,
    }
}

table! {
    professor_research_fields (prof_id, field_id) {
        prof_id -> Int4,
        field_id -> Int4,
    }
}

table! {
    professors (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    research_fields (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    student_applied_to (applicant_id, prof_id) {
        applicant_id -> Int4,
        prof_id -> Int4,
    }
}

joinable!(applicants -> research_fields (desired_field_id));
joinable!(professor_research_fields -> professors (prof_id));
joinable!(professor_research_fields -> research_fields (field_id));
joinable!(student_applied_to -> applicants (applicant_id));
joinable!(student_applied_to -> professors (prof_id));

allow_tables_to_appear_in_same_query!(
    applicants,
    professor_research_fields,
    professors,
    research_fields,
    student_applied_to,
);