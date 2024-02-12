CREATE TABLE "permission" (
    name VARCHAR(25) NOT NULL PRIMARY KEY,
    level SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "user" (
    username VARCHAR(64) NOT NULL PRIMARY KEY,
    email VARCHAR(320) NOT NULL UNIQUE,
    password VARCHAR(64) NOT NULL,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "user_permission" (
    user_id VARCHAR(64) NOT NULL UNIQUE,
    permission_id VARCHAR(25) NOT NULL UNIQUE,

    CONSTRAINT user_permission_pkey PRIMARY KEY (user_id, permission_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES "user"(username),
    CONSTRAINT fk_permission FOREIGN KEY (permission_id) REFERENCES "permission"(name)
);