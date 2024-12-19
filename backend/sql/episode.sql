-- name: find_episodes_by_media_id
select
    e.id,
    e.title,
    e.original_title,
    e.plot,
    e.nfo_path,
    e.video_file_path,
    e.subtitle_file_path,
    e.thumb_image_url,
    e.thumb_image,
    e.episodes_number,
    e.runtime,
    s.season_number,
    s.title as season_title
from
    episodes e
    join seasons s on e.season_id = s.id
where
    e.series_id = ?
order by
    e.episodes_number;
