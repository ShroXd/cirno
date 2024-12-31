-- name: find_media_by_id
select
    ts.id,
    ts.title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres
from
    tv_shows ts
    join tv_series_genres tsg on ts.id = tsg.series_id
    join genres g on tsg.genre_id = g.id
where
    ts.library_id = ?
    and ts.id = ?
group by
    ts.id,
    ts.title;

-- name: find_all_media
select
    ts.id,
    ts.title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres
from
    tv_shows ts
    join tv_series_genres tsg on ts.id = tsg.series_id
    join genres g on tsg.genre_id = g.id
where
    ts.library_id = ?
group by
    ts.id,
    ts.title;
