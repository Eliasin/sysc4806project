CREATE TABLE applicants (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    desired_field_id INTEGER REFERENCES research_fields NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    email TEXT NOT NULL,
    cv_path TEXT NOT NULL,
    diploma_path TEXT NOT NULL,
    grade_audit_path TEXT NOT NULL
);
