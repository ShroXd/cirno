# 📦 Node Environment Setup
FROM node:20-alpine AS frontend-builder

# 🛠️ Frontend Build Steps
# -------------------------------------------------------------------
WORKDIR /app/web
COPY web/package.json web/pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install
COPY web/ ./
RUN pnpm run build

# 📦 Rust Environment Setup
FROM rust:1.80 AS backend-builder

# Install required system libraries for GStreamer
RUN apt-get update && apt-get install -y \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    sqlite3

# Set the PKG_CONFIG_PATH environment variable
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 🛠️ Backend Build Steps
# ------------------------------------------------------------------
WORKDIR /app/backend
# COPY backend/Cargo.toml backend/Cargo.lock ./
# RUN mkdir src
# RUN echo "fn main() {}" > src/main.rs
# RUN cargo build --release
# RUN rm -rf src
COPY backend/ ./
RUN cargo build --release

# 🚀 Application Runner
FROM rust:1.80-slim AS runner

# Install required system libraries for GStreamer
RUN apt-get update && apt-get install -y \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    sqlite3

# Set the PKG_CONFIG_PATH environment variable
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 👤 User Setup
RUN useradd -m cirno

# 📁 Setup Working Directory and Permissions
WORKDIR /app
RUN mkdir logs && chown cirno:cirno logs && chown cirno:cirno ./

# 🔄 Copying Build Artifacts
COPY --from=frontend-builder /app/web/dist /app/web/dist
COPY --from=backend-builder /app/backend/target/release/cirno-backend /app/cirno-backend
COPY scripts/create_db.sh /app/create_db.sh

# 🌐 Network Configuration
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

# 📁 Database Configuration
ENV DATABASE_URL="sqlite://app/media_library.db"

# 🏃‍♂️ Running the Application
USER cirno

# 🏗️ Run the database creation script before starting the backend
CMD ["/bin/sh", "-c", "/app/create_db.sh /app/media_library.db && ./cirno-backend"]
