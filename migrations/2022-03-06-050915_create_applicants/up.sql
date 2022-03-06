CREATE TABLE applicants (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    desired_field_id INTEGER REFERENCES research_fields,
    name text,
    phone_number text,
    email text,
    cv_path text,
    diploma_path text,
    grade_audit_path text
);
