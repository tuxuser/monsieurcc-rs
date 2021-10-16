-- PS: SQLite is annoying
-- Create temp table
CREATE TABLE recipes_tmp AS (SELECT id, name, json_data, image_file FROM recipes);

--Drop old table
DROP recipes;

-- Rename temp table to former name
ALTER TABLE recipes_tmp RENAME TO recipes;
