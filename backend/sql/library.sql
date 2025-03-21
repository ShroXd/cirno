-- name: find_all_libraries
SELECT
    id,
    name,
    directory,
    category_id,
    item_count,
    last_scanned,
    current_status,
    auto_scan,
    error,
    storage_used,
    health_score,
    created_at,
    updated_at
FROM
    library
ORDER BY
    updated_at DESC;

-- name: find_library_by_id
SELECT
    id,
    name,
    directory,
    category_id,
    item_count,
    last_scanned,
    current_status,
    auto_scan,
    error,
    storage_used,
    health_score,
    created_at,
    updated_at
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
    join library_tv_shows lts on lts.tv_show_id = ts.id
where
    lts.library_id = ?
limit
    1;

-- name: save_library
INSERT
OR IGNORE INTO library (name, directory, category_id)
VALUES
    (?, ?, ?) RETURNING id;

-- name: populate_library_metadata
UPDATE library
SET
    item_count = ?,
    last_scanned = ?,
    current_status = ?,
    health_score = ?
WHERE
    id = ?;

-- name: delete_library
DELETE FROM library
WHERE
    id = ?;

-- name: update_library_by_id
UPDATE library
SET
    name = ?,
    directory = ?,
    category_id = ?
WHERE
    id = ?;
