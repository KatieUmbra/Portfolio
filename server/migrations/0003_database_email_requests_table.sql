CREATE TABLE email_requests(
    username    VARCHAR(20) NOT NULL,
    secret      VARCHAR(97) NOT NULL,
    -- Operation types
    --      0: verify account
    --      1: reset password
    operation   INT         NOT NULL,
    expiration  DATE        NOT NULL
);

ALTER TABLE email_requests
    ADD CONSTRAINT seclen CHECK (char_length(secret) = 97);
