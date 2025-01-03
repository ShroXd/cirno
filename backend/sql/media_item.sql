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

-- name: find_media_item_by_id
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
