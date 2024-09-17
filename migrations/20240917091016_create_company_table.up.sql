-- Add up migration script here
CREATE TABLE "company"
(
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    foundation_date   SMALLINT NOT NULL,
    name              TEXT UNIQUE NOT NULL,
    description       TEXT NOT NULL,
    url               TEXT UNIQUE,
    sector            TEXT NOT NULL CHECK (sector IN ('Digital', 'Marketing', 'Advertisement', 'Software', 'AI', 'Business', 'Music')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);