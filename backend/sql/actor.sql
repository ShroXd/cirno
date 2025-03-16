-- name: save_actor
INSERT
OR IGNORE INTO actors (name, role, thumb, profile, tmdb_id)
VALUES
    (?, ?, ?, ?, ?) RETURNING id;

-- name: save_actor_to_tv_show
INSERT
OR IGNORE INTO tv_show_actors (tv_show_id, actor_id)
VALUES
    (?, ?);

-- name: find_actors_by_media_id
SELECT 
    a.name,
    a.role,
    a.thumb,
    a.profile,
    a.tmdb_id
FROM actors a
JOIN tv_show_actors tsa ON a.id = tsa.actor_id
WHERE tsa.tv_show_id = ?;
