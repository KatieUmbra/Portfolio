CREATE TABLE users (
	username	    VARCHAR(20) NOT NULL UNIQUE PRIMARY KEY,
    displayUsername VARCHAR(20) NOT NULL UNIQUE,
	displayName	    VARCHAR(30) NOT NULL,
	email		    VARCHAR     NOT NULL UNIQUE,
	password	    VARCHAR     NOT NULL,
    verified        BOOL        NOT NULL DEFAULT FALSE
);
