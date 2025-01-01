-- name: find_seasons_by_tv_show_id
SELECT
    id,
    season_number,
    title
FROM
    seasons
WHERE
    tv_show_id = ?
ORDER BY
    season_number ASC;

-- name: save_season
INSERT INTO
    seasons (tv_show_id, season_number, title, plot, nfo_path)
VALUES
    (?, ?, ?, ?, ?) ON CONFLICT (tv_show_id, season_number) DO
UPDATE
SET
    id = id RETURNING id;
