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

# 🛠️ Backend Build Steps
# ------------------------------------------------------------------
WORKDIR /app/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY backend/ ./
RUN cargo build --release

# 🚀 Application Runner
FROM rust:1.80-slim AS runner

# 👤 User Setup
RUN useradd -m cirno

# 📁 Setup Working Directory and Permissions
WORKDIR /app
RUN mkdir logs && chown cirno:cirno logs

# 🔄 Copying Build Artifacts
COPY --from=frontend-builder /app/web/dist /app/web/dist
COPY --from=backend-builder /app/backend/target/release/cirno-backend /app/cirno-backend

# 🌐 Network Configuration
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

# 🏃‍♂️ Running the Application
USER cirno
CMD ["./cirno-backend"]
