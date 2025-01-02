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
