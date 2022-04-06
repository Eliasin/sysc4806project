CREATE TABLE applicant_logins (
    id INTEGER NOT NULL REFERENCES applicants,
    username text NOT NULL, 
    bcrypt_hash char(60) NOT NULL, 
    PRIMARY KEY (id));

CREATE TABLE professor_logins (
    id INTEGER NOT NULL REFERENCES professors,
    username text NOT NULL,
    bcrypt_hash char(60) NOT NULL, 
    PRIMARY KEY (id));

CREATE TABLE admin_logins (
    username text PRIMARY KEY, 
    bcrypt_hash char(60) NOT NULL);