-- create users table
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    roles TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- populate users table
INSERT INTO users (
        name,
        username,
        email,
        password_hash,
        active,
        roles,
        created_at,
        updated_at
    )
VALUES (
        'Fake Admin',
        'fake_admin',
        'fake_admin@example.com',
        -- password is password
        '$argon2id$v=19$m=15360,t=2,p=1$2KWoDzmay5Mi8JCqV7FZGA$mS7z4djVhAMkpfcRLRQ8krJH1dQSJkJ4ent6vrNvM/0',
        'true',
        'admin',
        now(),
        now()
    );