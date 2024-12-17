-- name: get_media_libraries
SELECT
    id,
    name,
    category_id
FROM
    media_library;

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
