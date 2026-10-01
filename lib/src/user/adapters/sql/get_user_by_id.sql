SELECT *
FROM users
WHERE id = $1
  AND deleted_at IS NULL;
