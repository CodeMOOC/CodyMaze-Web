# ---
# First stage (build)
FROM rust:1.37 AS build
WORKDIR /usr/src/codymaze-web

# Dependency compilation
COPY Cargo.* ./
COPY fakeSrc/ src/
RUN cargo build --release

# Replace with real project
RUN rm src/main.rs
COPY src/ src/
RUN cargo build --release --target-dir /usr/src/codymaze-web/build

# ---
# Second stage (execution)
FROM debian:jessie-slim AS runtime

WORKDIR /app
COPY --from=build /usr/src/codymaze-web/build/release/codymaze-web codymaze-web
COPY ./static/ static/
COPY ./templates templates/

EXPOSE 8088

# Drop privileges
USER 1000

CMD ["/app/codymaze-web"]
