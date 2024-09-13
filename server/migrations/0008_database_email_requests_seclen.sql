ALTER TABLE email_requests
    DROP CONSTRAINT seclen,
    ALTER COLUMN secret TYPE VARCHAR(60),
    ADD CONSTRAINT seclen CHECK (char_length(secret) = 60);
