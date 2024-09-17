-- Add up migration script here
CREATE TABLE "user"
(
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name           TEXT NOT NULL,
    surname        TEXT NOT NULL,
    email          TEXT UNIQUE NOT NULL,
    role           TEXT NOT NULL CHECK (role IN ('ADMIN', 'MODERATOR', 'USER')),
    password_hash  TEXT NOT NULL,
    reset_token TEXT,
    reset_sent_at TIMESTAMPTZ,
    email_verification_token TEXT,
    email_verification_sent_at TIMESTAMPTZ,
    email_verified_at TIMESTAMPTZ,
    blocked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
