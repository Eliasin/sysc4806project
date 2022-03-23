ALTER TABLE applicants DROP COLUMN cv_blob_id;
ALTER TABLE applicants DROP COLUMN diploma_blob_id;
ALTER TABLE applicants DROP COLUMN grade_audit_blob_id;

DROP TABLE applicant_blobs;

ALTER TABLE applicants ADD COLUMN cv_path TEXT NOT NULL;
ALTER TABLE applicants ADD COLUMN diploma_path TEXT NOT NULL;
ALTER TABLE applicants ADD COLUMN grade_audit_path TEXT NOT NULL;
