UPDATE users
SET
  deleted_at = now(),
  updated_at = now()
WHERE email = $1
  AND deleted_at IS NULL
RETURNING *;
