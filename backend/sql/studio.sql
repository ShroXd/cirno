-- name: save_studio
INSERT
OR IGNORE INTO studios (name)
VALUES
    (?) ON CONFLICT (name) DO
UPDATE
SET
    id = id RETURNING id;

-- name: save_tv_show_studio
INSERT
OR IGNORE INTO tv_show_studios (tv_show_id, studio_id)
VALUES
    (?, ?);
