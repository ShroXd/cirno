-- name: find_all_libraries
SELECT
    id,
    name,
    category_id
FROM
    library
ORDER BY
    updated_at DESC;

-- name: find_library_by_id
SELECT
    id,
    name,
    category_id
FROM
    library
WHERE
    id = ?;

-- name: get_library_posters
select
    ts.id,
    ts.poster_path
from
    tv_shows ts
where
    ts.library_id = ?
limit
    1;

-- name: save_library
INSERT
OR IGNORE INTO library (name, directory, category_id)
VALUES
    (?, ?, ?) RETURNING id;

-- name: delete_library
DELETE FROM library
WHERE
    id = ?;
