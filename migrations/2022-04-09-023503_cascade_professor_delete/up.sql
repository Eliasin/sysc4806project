ALTER TABLE student_applied_to 
    DROP CONSTRAINT student_applied_to_prof_id_fkey,
    ADD CONSTRAINT student_applied_to_prof_id_fkey
    FOREIGN KEY (prof_id)
    REFERENCES professors(id)
    ON DELETE CASCADE;

ALTER TABLE student_applied_to
    DROP CONSTRAINT student_applied_to_applicant_id_fkey,
    ADD CONSTRAINT student_applied_to_applicant_id_fkey
    FOREIGN KEY (applicant_id)
    REFERENCES applicants(id)
    ON DELETE CASCADE;