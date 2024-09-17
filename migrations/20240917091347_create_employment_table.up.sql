-- Add up migration script here
CREATE TABLE "employment" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    company_id UUID NOT NULL,
    position TEXT NOT NULL CHECK (position IN ('CEO', 'Manager', 'WhiteCollar', 'BlueCollar')),
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES "company" (id) ON DELETE CASCADE
);