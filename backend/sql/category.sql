-- name: check_category_exists
SELECT
    id
FROM
    category_mapping
WHERE
    id = ?;
