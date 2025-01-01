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
    e.episode_number,
    e.runtime,
    s.season_number,
    s.title as season_title
from
    episodes e
    join seasons s on e.season_id = s.id
    join tv_shows t on s.tv_show_id = t.id
where
    t.id = ?
    and t.library_id = ?
    and e.deleted_at is null
order by
    s.season_number,
    e.episode_number;

-- name: save_episode
INSERT
OR IGNORE INTO episodes (
    season_id,
    title,
    original_title,
    plot,
    nfo_path,
    video_file_path,
    subtitle_file_path,
    thumb_image_url,
    thumb_image,
    episode_number,
    runtime
)
VALUES
    (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
