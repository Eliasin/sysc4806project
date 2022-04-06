table! {
    admin_logins (username) {
        username -> Text,
        bcrypt_hash -> Bpchar,
    }
}

table! {
    applicant_blobs (id) {
        id -> Int4,
        data_blob -> Bytea,
    }
}

table! {
    applicant_logins (id) {
        id -> Int4,
        username -> Text,
        bcrypt_hash -> Bpchar,
    }
}

table! {
    applicants (id) {
        id -> Int4,
        desired_field_id -> Int4,
        name -> Text,
        phone_number -> Text,
        email -> Text,
        cv_blob_id -> Nullable<Int4>,
        diploma_blob_id -> Nullable<Int4>,
        grade_audit_blob_id -> Nullable<Int4>,
    }
}

table! {
    professor_logins (id) {
        id -> Int4,
        username -> Text,
        bcrypt_hash -> Bpchar,
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
        name -> Text,
    }
}

table! {
    research_fields (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    student_applied_to (applicant_id, prof_id) {
        applicant_id -> Int4,
        prof_id -> Int4,
        status -> Text,
    }
}

joinable!(applicant_logins -> applicants (id));
joinable!(applicants -> research_fields (desired_field_id));
joinable!(professor_logins -> professors (id));
joinable!(professor_research_fields -> professors (prof_id));
joinable!(professor_research_fields -> research_fields (field_id));
joinable!(student_applied_to -> applicants (applicant_id));
joinable!(student_applied_to -> professors (prof_id));

allow_tables_to_appear_in_same_query!(
    admin_logins,
    applicant_blobs,
    applicant_logins,
    applicants,
    professor_logins,
    professor_research_fields,
    professors,
    research_fields,
    student_applied_to,
);
