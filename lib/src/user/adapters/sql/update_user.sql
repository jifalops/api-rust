UPDATE users
SET
  email_verified = $2,
  name = COALESCE($3, name),
  photo_url = COALESCE($4, photo_url),
  updated_at = now()
WHERE id = $1
  AND deleted_at IS NULL
RETURNING *;
