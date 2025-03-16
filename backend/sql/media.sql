-- name: find_media_by_id
select
    ts.id,
    ts.title,
    ts.original_title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.premiered,
    ts.rating,
    ts.runtime,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres,
    group_concat (s.name, ', ') AS studios
from
    tv_shows ts
    left join library_tv_shows lts on lts.tv_show_id = ts.id
    left join tv_show_genres tsg on ts.id = tsg.tv_show_id
    left join genres g on tsg.genre_id = g.id
    left join tv_show_studios tss on ts.id = tss.tv_show_id
    left join studios s on tss.studio_id = s.id
where
    lts.library_id = ?
    and ts.id = ?
group by
    ts.id,
    ts.title;

-- name: find_all_media
select
    ts.id,
    ts.title,
    ts.original_title,
    ts.poster_path,
    ts.fanart_path,
    ts.country,
    ts.premiered,
    ts.rating,
    ts.runtime,
    ts.year,
    ts.plot,
    group_concat (g.name, ', ') AS genres,
    group_concat (s.name, ', ') AS studios
from
    tv_shows ts
    left join library_tv_shows lts on lts.tv_show_id = ts.id
    left join tv_show_genres tsg on ts.id = tsg.tv_show_id
    left join genres g on tsg.genre_id = g.id
    left join tv_show_studios tss on ts.id = tss.tv_show_id
    left join studios s on tss.studio_id = s.id
where
    lts.library_id = ?
group by
    ts.id,
    ts.title;
