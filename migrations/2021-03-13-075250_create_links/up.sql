CREATE TABLE links (
    id BIGSERIAL PRIMARY KEY,
    url VARCHAR NOT NULL,
    redirect_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    expired_at TIMESTAMP NULL
)
