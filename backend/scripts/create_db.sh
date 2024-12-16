#!/bin/bash

# Get the project root path dynamically (assumes the script is located in a subfolder of the project root)
PROJECT_ROOT=$(dirname "$(realpath "$0")")/../..
SCHEMA_FILE="$PROJECT_ROOT/backend/sql/schema.sql"

# Check if the first argument is provided, if not use the default path
if [ -z "$1" ]; then
    DB_PATH="$PROJECT_ROOT/backend/media_library.db"
else
    DB_PATH="$1"
fi

# Check if schema file exists
if [ ! -f "$SCHEMA_FILE" ]; then
    echo "❌ Schema file not found at $SCHEMA_FILE"
    exit 1
fi

# Remove the existing database file if it exists
if [ -f "$DB_PATH" ]; then
    echo "🗑️ Removing existing database..."
    rm "$DB_PATH"
fi

# Create a new, empty database file
echo "✨ Creating a new database..."
sqlite3 "$DB_PATH" "VACUUM;"

# Apply the schema
echo "📦 Preparing the database schema..."
sqlite3 "$DB_PATH" ".read $SCHEMA_FILE"

echo "✅ Database setup completed."
