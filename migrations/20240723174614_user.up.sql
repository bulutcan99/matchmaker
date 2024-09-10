CREATE TABLE "user"
(
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name           TEXT NOT NULL,
    surname        TEXT NOT NULL,
    email          TEXT UNIQUE NOT NULL,
    role           TEXT NOT NULL CHECK (role IN ('ADMIN', 'MODERATOR', 'USER')),
    password_hash  TEXT NOT NULL,
    is_blocked     BOOLEAN NOT NULL,
    is_verified    BOOLEAN NOT NULL,
    google_oauth_id TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);