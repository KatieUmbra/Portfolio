ALTER TABLE posts
    ALTER COLUMN creation TYPE TIMESTAMPTZ;

ALTER TABLE comments
    ALTER COLUMN creation TYPE TIMESTAMPTZ;
