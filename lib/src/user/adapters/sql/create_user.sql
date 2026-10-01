INSERT INTO users (id, email, email_verified, password_hash, name, photo_url)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;
