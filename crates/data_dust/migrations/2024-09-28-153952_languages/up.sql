-- Your SQL goes here
CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(100) NOT NULL,
    compiler VARCHAR(255) NOT NULL,
    bit_size INT,
    additional_info VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Create index on name and version for faster lookups
CREATE INDEX idx_languages_name_version ON languages(name, version);

-- Add a trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_languages_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_languages_timestamp
BEFORE UPDATE ON languages
FOR EACH ROW EXECUTE FUNCTION update_languages_timestamp();
