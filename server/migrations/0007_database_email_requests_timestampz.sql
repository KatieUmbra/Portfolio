SET timezone = "UTC";
ALTER TABLE email_requests
    ALTER COLUMN expiration TYPE TIMESTAMPTZ;
