-- name: save_genre
INSERT
OR IGNORE INTO genres (name)
VALUES
    (?) ON CONFLICT (name) DO
UPDATE
SET
    id = id RETURNING id;

-- name: save_tv_show_genre
INSERT
OR IGNORE INTO tv_show_genres (tv_show_id, genre_id)
VALUES
    (?, ?);
