-- name: find_all_media_items
SELECT
    ts.id,
    ts.title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres
FROM
    tv_shows ts
    JOIN tv_show_genres tsg ON ts.id = tsg.tv_show_id
    JOIN genres g ON tsg.genre_id = g.id
GROUP BY
    ts.id,
    ts.title;

-- name: find_media_item_by_library_id
SELECT
    ts.id,
    ts.title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres
FROM
    tv_shows ts
    JOIN tv_show_genres tsg ON ts.id = tsg.tv_show_id
    JOIN genres g ON tsg.genre_id = g.id
WHERE
    ts.library_id = ?
GROUP BY
    ts.id,
    ts.title;

-- name: find_media_item_by_id
SELECT
    ts.id,
    ts.title,
    ts.original_title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.year,
    ts.premiered,
    ts.rating,
    ts.runtime,
    ts.plot,
    group_concat (DISTINCT g.name) AS genres,
    group_concat (DISTINCT s.name) AS studios
FROM
    tv_shows ts
    LEFT JOIN tv_show_genres tsg ON ts.id = tsg.tv_show_id
    LEFT JOIN genres g ON tsg.genre_id = g.id
    LEFT JOIN tv_show_studios tss ON ts.id = tss.tv_show_id
    LEFT JOIN studios s ON tss.studio_id = s.id
WHERE
    ts.id = ?
GROUP BY
    ts.id,
    ts.title;
