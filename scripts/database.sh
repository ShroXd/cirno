#!/bin/bash

# Get the project root path dynamically (assumes the script is located in a subfolder of the project root)
PROJECT_ROOT=$(dirname "$(realpath "$0")")/..

# Define the database path relative to the project root
DB_PATH="$PROJECT_ROOT/media_library.db"

# Remove the existing database file if it exists
if [ -f "$DB_PATH" ]; then
    echo "🗑️ Removing existing database..."
    rm "$DB_PATH"
fi

# Create a new, empty database file
echo "✨ Creating a new database..."
sqlite3 "$DB_PATH" "VACUUM;"

# Prepare the database with the required schema
echo "📦 Preparing the database schema..."
sqlite3 "$DB_PATH" <<EOF
CREATE TABLE IF NOT EXISTS tv_series (
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
    tvdb_id TEXT
);

CREATE TABLE IF NOT EXISTS seasons (
    id INTEGER PRIMARY KEY,
    series_id INTEGER NOT NULL,
    season_number INTEGER,
    title TEXT,
    plot TEXT,
    nfo_path TEXT,
    FOREIGN KEY(series_id) REFERENCES tv_series(id)
);

CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY,
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
    FOREIGN KEY(season_id) REFERENCES seasons(id),
    FOREIGN KEY(series_id) REFERENCES tv_series(id)
);

CREATE TABLE IF NOT EXISTS genres (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS tv_series_genres (
    series_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,
    PRIMARY KEY (series_id, genre_id),
    FOREIGN KEY(series_id) REFERENCES tv_series(id),
    FOREIGN KEY(genre_id) REFERENCES genres(id)
);

CREATE TABLE IF NOT EXISTS actors (
    id INTEGER PRIMARY KEY,
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
    FOREIGN KEY(series_id) REFERENCES tv_series(id),
    FOREIGN KEY(actor_id) REFERENCES actors(id)
);
EOF

echo "✅ Database setup completed."
