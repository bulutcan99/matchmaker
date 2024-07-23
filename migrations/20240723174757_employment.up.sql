CREATE TYPE "position" AS ENUM ('CEO', 'Manager', 'WhiteCollar', 'BlueCollar');

CREATE TABLE "employment" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    company_id UUID NOT NULL,
    position "position" NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES "company" (id) ON DELETE CASCADE
);