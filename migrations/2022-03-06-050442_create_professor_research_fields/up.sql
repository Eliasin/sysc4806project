CREATE TABLE professor_research_fields (
    prof_id INTEGER REFERENCES professors NOT NULL,
    field_id INTEGER REFERENCES research_fields NOT NULL,
    PRIMARY KEY (prof_id, field_id)
);
