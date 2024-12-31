CREATE TABLE IF NOT EXISTS tv_shows (
    id INTEGER PRIMARY KEY,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (library_id) REFERENCES library (id)
);

CREATE TABLE IF NOT EXISTS seasons (
    id INTEGER PRIMARY KEY,
    tv_show_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    title TEXT,
    plot TEXT,
    nfo_path TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id),
    UNIQUE (tv_show_id, season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY,
    season_id INTEGER NOT NULL,
    tv_show_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    original_title TEXT,
    plot TEXT,
    nfo_path TEXT,
    video_file_path TEXT NOT NULL,
    subtitle_file_path TEXT,
    thumb_image_url TEXT,
    thumb_image TEXT,
    episode_number INTEGER,
    runtime INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (season_id) REFERENCES seasons (id),
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id),
    UNIQUE (season_id, episode_number)
);

CREATE TABLE IF NOT EXISTS genres (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS tv_series_genres (
    series_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (series_id, genre_id),
    FOREIGN KEY (series_id) REFERENCES tv_shows (id),
    FOREIGN KEY (genre_id) REFERENCES genres (id)
);

CREATE TABLE IF NOT EXISTS actors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    role TEXT,
    thumb TEXT,
    profile TEXT,
    tmdb_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS tv_series_actors (
    series_id INTEGER NOT NULL,
    actor_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (series_id, actor_id),
    FOREIGN KEY (series_id) REFERENCES tv_shows (id),
    FOREIGN KEY (actor_id) REFERENCES actors (id)
);

CREATE TABLE category_mapping (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

INSERT INTO
    category_mapping (id, name, created_at, updated_at)
VALUES
    (1, 'Movie', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    category_mapping (id, name, created_at, updated_at)
VALUES
    (2, 'TVShow', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    category_mapping (id, name, created_at, updated_at)
VALUES
    (
        3,
        'Animation',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS library (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    directory TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (category_id) REFERENCES category_mapping (id) ON DELETE SET NULL ON UPDATE CASCADE
)
