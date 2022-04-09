ALTER TABLE student_applied_to 
    DROP constraint student_applied_to_prof_id_fkey,
    ADD constraint student_applied_to_prof_id_fkey
    foreign key (prof_id)
    references professors(id);

ALTER TABLE student_applied_to 
    DROP constraint student_applied_to_applicant_id_fkey,
    ADD constraint student_applied_to_applicant_id_fkey
    foreign key (applicant_id)
    references applicants(id);