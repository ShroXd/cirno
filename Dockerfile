# 🚧 Build Frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /app/web
COPY web/package.json web/pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install
COPY web/ ./
RUN pnpm run build

# 🦀 Build Backend (Rust)
FROM rust:1.80 AS backend-builder

# 📦 Install GStreamer Dependencies
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

ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

WORKDIR /app/backend
COPY backend/ ./
RUN cargo build --release

# 🚀 Final Stage: Application Runner
FROM rust:1.80-slim AS runner

# 🔧 Install GStreamer Dependencies
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

ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 👤 Set up User and Permissions
RUN useradd -m -u 1026 -g users cirno

WORKDIR /app
RUN mkdir -p /app/backend /app/logs /app/data /app/config /app/tmp && \
    chown -R cirno:users /app && \
    chmod -R 755 /app && \
    chmod 770 /app/data

# 📤 Copy Build Artifacts
COPY --from=frontend-builder /app/web/dist /app/web/dist
COPY --from=backend-builder /app/backend/target/release/cirno-backend /app/cirno-backend
COPY backend/scripts/create_db.sh /app/backend/scripts/create_db.sh
COPY backend/sql /app/backend/sql
RUN chown -R cirno:users /app

# 🌍 Expose Port and Set Environment Variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV DATABASE_URL="sqlite:///app/backend/database.db"
EXPOSE 8000

# 🏁 Switch to Non-Root User and Start Application
USER cirno
CMD ["/bin/sh", "-c", "./backend/scripts/create_db.sh && ./cirno-backend"]
