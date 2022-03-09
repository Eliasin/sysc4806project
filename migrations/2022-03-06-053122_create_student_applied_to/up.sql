CREATE TABLE student_applied_to (
    applicant_id INTEGER REFERENCES applicants NOT NULL,
    prof_id INTEGER REFERENCES professors NOT NULL,
    PRIMARY KEY (applicant_id, prof_id)
);
