CREATE TABLE posts (
    id              SERIAL      NOT NULL UNIQUE PRIMARY KEY,
    creator         VARCHAR(20) NOT NULL REFERENCES users(username),
    description     VARCHAR(100),
    title           VARCHAR(50) NOT NULL,
    creation        DATE        NOT NULL,
    likes           INT         NOT NULL DEFAULT 0
);
