CREATE TABLE IF NOT EXISTS tv_series (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL UNIQUE,
    nfo_path TEXT,
    poster_path TEXT,
    fanart_path TEXT,
    country TEXT,
    year INTEGER,
    plot TEXT,
    tmdb_id TEXT,
    imdb_id TEXT,
    wikidata_id TEXT,
    tvdb_id TEXT,
    library_id INTEGER,
    FOREIGN KEY (library_id) REFERENCES library (id)
);

CREATE TABLE IF NOT EXISTS seasons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    series_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    title TEXT,
    plot TEXT,
    nfo_path TEXT,
    FOREIGN KEY (series_id) REFERENCES tv_series (id),
    UNIQUE (series_id, season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season_id INTEGER NOT NULL,
    series_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    original_title TEXT,
    plot TEXT,
    nfo_path TEXT,
    video_file_path TEXT NOT NULL,
    subtitle_file_path TEXT,
    thumb_image_url TEXT,
    thumb_image TEXT,
    season_number INTEGER,
    episodes_number INTEGER,
    runtime INTEGER,
    FOREIGN KEY (season_id) REFERENCES seasons (id),
    FOREIGN KEY (series_id) REFERENCES tv_series (id)
);

CREATE TABLE IF NOT EXISTS genres (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS tv_series_genres (
    series_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,
    PRIMARY KEY (series_id, genre_id),
    FOREIGN KEY (series_id) REFERENCES tv_series (id),
    FOREIGN KEY (genre_id) REFERENCES genres (id)
);

CREATE TABLE IF NOT EXISTS actors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    role TEXT,
    thumb TEXT,
    profile TEXT,
    tmdb_id TEXT
);

CREATE TABLE IF NOT EXISTS tv_series_actors (
    series_id INTEGER NOT NULL,
    actor_id INTEGER NOT NULL,
    PRIMARY KEY (series_id, actor_id),
    FOREIGN KEY (series_id) REFERENCES tv_series (id),
    FOREIGN KEY (actor_id) REFERENCES actors (id)
);

CREATE TABLE category_mapping (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE);

INSERT INTO
    category_mapping (id, name)
VALUES
    (1, 'Movie');

INSERT INTO
    category_mapping (id, name)
VALUES
    (2, 'TVShow');

INSERT INTO
    category_mapping (id, name)
VALUES
    (3, 'Animation');

CREATE TABLE IF NOT EXISTS library (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    directory TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (category_id) REFERENCES category_mapping (id) ON DELETE SET NULL ON UPDATE CASCADE
)
