CREATE TABLE comments (
    id          SERIAL      NOT NULL UNIQUE PRIMARY KEY,
    creator     VARCHAR(20) NOT NULL,
    post        INT         NOT NULL REFERENCES posts(id),
    parent_id   INT                  REFERENCES comments(id),
    creation    DATE        NOT NULL,
    content     VARCHAR(200)NOT NULL,
    likes       INT         NOT NULL DEFAULT 0
);
