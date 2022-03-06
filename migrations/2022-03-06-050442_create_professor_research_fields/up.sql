CREATE TABLE professor_research_fields (
    prof_id INTEGER REFERENCES professors,
    field_id INTEGER REFERENCES research_fields,
    PRIMARY KEY (prof_id, field_id)
);
