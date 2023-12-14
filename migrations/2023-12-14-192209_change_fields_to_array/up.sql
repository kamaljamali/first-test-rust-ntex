-- Your SQL goes here
ALTER TABLE students
ALTER COLUMN fields TYPE VARCHAR[]
USING array[fields]::VARCHAR[];