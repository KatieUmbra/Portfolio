ALTER TABLE email_requests
    ADD CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username);
