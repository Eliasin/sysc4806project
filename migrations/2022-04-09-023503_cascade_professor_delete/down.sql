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

ALTER TABLE professor_logins 
    DROP constraint professor_logins_id_fkey,
    ADD constraint professor_logins_id_fkey
    foreign key (id)
    references professors(id);

ALTER TABLE applicant_logins 
    DROP constraint applicant_logins_id_fkey,
    ADD constraint applicant_logins_id_fkey
    foreign key (id)
    references applicants(id);