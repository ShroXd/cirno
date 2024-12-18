-- name: find_all_media_libraries
SELECT
    id,
    name,
    category_id
FROM
    media_library
ORDER BY
    updated_at DESC;

-- name: find_media_library_by_id
SELECT
    id,
    name,
    category_id
FROM
    media_library
WHERE
    id = ?;

-- name: get_media_library_posters
select
    ts.id,
    ts.poster_path
from
    tv_series ts
where
    ts.media_library_id = ?
limit
    1;

-- name: save_media_library
INSERT
OR IGNORE INTO media_library (name, directory, category_id)
VALUES
    (?, ?, ?) RETURNING id;
