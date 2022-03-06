CREATE TABLE student_applied_to (
    applicant_id INTEGER REFERENCES applicants,
    prof_id INTEGER REFERENCES professors,
    PRIMARY KEY (applicant_id, prof_id)
);
