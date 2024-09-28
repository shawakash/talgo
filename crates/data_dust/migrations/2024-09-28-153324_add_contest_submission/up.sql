-- Your SQL goes here
ALTER TABLE submissions
ADD COLUMN contest_sub BOOLEAN NOT NULL DEFAULT false;

-- Create a function to update contest_sub
CREATE OR REPLACE FUNCTION update_contest_sub()
RETURNS TRIGGER AS $$
BEGIN
    NEW.contest_sub := EXISTS (
        SELECT 1 FROM contests
        WHERE contests.id = NEW.contest_id
        AND NEW.submitted_at BETWEEN contests.start_time AND contests.end_time
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the function before insert or update
CREATE TRIGGER set_contest_sub
BEFORE INSERT OR UPDATE ON submissions
FOR EACH ROW EXECUTE FUNCTION update_contest_sub();
