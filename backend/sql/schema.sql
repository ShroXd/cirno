CREATE TABLE IF NOT EXISTS library (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    directory TEXT NOT NULL,
    category_id INTEGER,
    item_count INTEGER DEFAULT 0,
    last_scanned TEXT DEFAULT NULL,
    current_status INTEGER DEFAULT 1,
    auto_scan BOOLEAN DEFAULT TRUE,
    error TEXT DEFAULT NULL,
    storage_used INTEGER DEFAULT 0,
    health_score INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (category_id) REFERENCES category_mapping (id) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS tv_shows (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL UNIQUE,
    original_title TEXT,
    nfo_path TEXT,
    poster_path TEXT,
    fanart_path TEXT,
    country TEXT,
    year INTEGER,
    premiered TEXT,
    rating INTEGER,
    runtime INTEGER,
    plot TEXT,
    tmdb_id TEXT,
    imdb_id TEXT,
    wikidata_id TEXT,
    tvdb_id TEXT,
    reference_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS library_tv_shows (
    library_id INTEGER NOT NULL,
    tv_show_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (library_id, tv_show_id),
    FOREIGN KEY (library_id) REFERENCES library (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id) ON DELETE CASCADE ON UPDATE CASCADE
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
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE (tv_show_id, season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY,
    season_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    original_title TEXT,
    plot TEXT,
    nfo_path TEXT,
    video_file_path TEXT NOT NULL UNIQUE,
    subtitle_file_path TEXT,
    thumb_image_url TEXT,
    thumb_image TEXT,
    episode_number INTEGER NOT NULL,
    runtime INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (season_id) REFERENCES seasons (id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE (season_id, episode_number)
);

CREATE TABLE IF NOT EXISTS genres (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS tv_show_genres (
    tv_show_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (tv_show_id, genre_id),
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (genre_id) REFERENCES genres (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS studios (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    reference_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS tv_show_studios (
    tv_show_id INTEGER NOT NULL,
    studio_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (tv_show_id, studio_id),
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (studio_id) REFERENCES studios (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS actors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    role TEXT,
    thumb TEXT,
    profile TEXT,
    tmdb_id TEXT,
    reference_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS tv_show_actors (
    tv_show_id INTEGER NOT NULL,
    actor_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (tv_show_id, actor_id),
    FOREIGN KEY (tv_show_id) REFERENCES tv_shows (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (actor_id) REFERENCES actors (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE category_mapping (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE IF NOT EXISTS library_status (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TRIGGER increment_library_tv_show_reference_count AFTER INSERT ON library_tv_shows BEGIN
UPDATE tv_shows
SET
    reference_count = reference_count + 1
WHERE
    id = NEW.tv_show_id;

END;

CREATE TRIGGER decrement_library_tv_show_reference_count AFTER DELETE ON library_tv_shows BEGIN
UPDATE tv_shows
SET
    reference_count = reference_count - 1
WHERE
    id = OLD.tv_show_id;

DELETE FROM tv_shows
WHERE
    id = OLD.tv_show_id
    AND reference_count = 0;

END;

CREATE TRIGGER increment_genre_reference_count AFTER INSERT ON tv_show_genres BEGIN
UPDATE genres
SET
    reference_count = reference_count + 1
WHERE
    id = NEW.genre_id;

END;

CREATE TRIGGER decrement_genre_reference_count AFTER DELETE ON tv_show_genres BEGIN
UPDATE genres
SET
    reference_count = reference_count - 1
WHERE
    id = OLD.genre_id;

DELETE FROM genres
WHERE
    id = OLD.genre_id
    AND reference_count = 0;

END;

CREATE TRIGGER increment_studio_reference_count AFTER INSERT ON tv_show_studios BEGIN
UPDATE studios
SET
    reference_count = reference_count + 1
WHERE
    id = NEW.studio_id;

END;

CREATE TRIGGER decrement_studio_reference_count AFTER DELETE ON tv_show_studios BEGIN
UPDATE studios
SET
    reference_count = reference_count - 1
WHERE
    id = OLD.studio_id;

DELETE FROM studios
WHERE
    id = OLD.studio_id
    AND reference_count = 0;

END;


CREATE TRIGGER increment_actor_reference_count AFTER INSERT ON tv_show_actors BEGIN
UPDATE actors
SET
    reference_count = reference_count + 1
WHERE
    id = NEW.actor_id;

END;

CREATE TRIGGER decrement_actor_reference_count AFTER DELETE ON tv_show_actors BEGIN
UPDATE actors
SET
    reference_count = reference_count - 1
WHERE
    id = OLD.actor_id;

DELETE FROM actors
WHERE
    id = OLD.actor_id
    AND reference_count = 0;

END;

-- Category Mapping

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

-- Library Status

INSERT INTO
    library_status (id, name, created_at, updated_at)
VALUES
    (1, 'Pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    library_status (id, name, created_at, updated_at)
VALUES
    (2, 'Scanning', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    library_status (id, name, created_at, updated_at)
VALUES
    (3, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    library_status (id, name, created_at, updated_at)
VALUES
    (4, 'Inactive', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO
    library_status (id, name, created_at, updated_at)
VALUES
    (5, 'Error', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
