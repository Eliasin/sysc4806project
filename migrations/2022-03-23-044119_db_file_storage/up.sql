ALTER TABLE applicants DROP COLUMN cv_path;
ALTER TABLE applicants DROP COLUMN diploma_path;
ALTER TABLE applicants DROP COLUMN grade_audit_path;

CREATE TABLE applicant_blobs (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    data_blob bytea NOT NULL
);

ALTER TABLE applicants ADD COLUMN cv_blob_id INTEGER REFERENCES applicant_blobs;
ALTER TABLE applicants ADD COLUMN diploma_blob_id INTEGER REFERENCES applicant_blobs;
ALTER TABLE applicants ADD COLUMN grade_audit_blob_id INTEGER REFERENCES applicant_blobs;
