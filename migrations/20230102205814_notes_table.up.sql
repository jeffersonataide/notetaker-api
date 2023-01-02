-- Add up migration script here
CREATE TABLE IF NOT EXISTS notes (
  id BIGSERIAL PRIMARY KEY,
  content TEXT NOT NULL
)
