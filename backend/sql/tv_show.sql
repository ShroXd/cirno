-- name: save_tv_show
INSERT INTO
    tv_shows (
        title,
        nfo_path,
        poster_path,
        fanart_path,
        country,
        year,
        plot,
        tmdb_id,
        imdb_id,
        wikidata_id,
        tvdb_id,
        library_id
    )
VALUES
    (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT (title) DO
UPDATE
SET
    id = id RETURNING id;
