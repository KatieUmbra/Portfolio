ALTER TABLE users
    ALTER COLUMN email TYPE VARCHAR(50),
    -- 97 is the exact size of the hashed password
    ALTER COLUMN password TYPE VARCHAR(97),
    ADD CONSTRAINT pwdlen CHECK (char_length(password) = 97),
    -- Verified levels:
    --     0: admin (can edit blog entries and manage stuff)
    --     1: user  (can comment and like and contact)
    --     2: guest (can like and contact)
    ALTER COLUMN verified DROP DEFAULT,
    ALTER COLUMN verified TYPE INT USING CASE WHEN verified=FALSE THEN 2 ELSE 1 END,
    ALTER COLUMN verified SET DEFAULT 2;
