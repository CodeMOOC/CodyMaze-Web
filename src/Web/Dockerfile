# ---
# First stage (build)
FROM rust:1.37 AS build
WORKDIR /usr/src/codymaze-web

# Setup fully static compilation target
#RUN apt update && apt install -y \
#    gcc-arm-linux-gnueabihf && \
#    rm -rf /var/lib/apt/lists/*
#RUN rustup target add x86_64-unknown-linux-musl

# Dependency compilation
COPY Cargo.* ./
COPY fakeSrc/ src/
RUN cargo build --release

# --target x86_64-unknown-linux-musl

# Replace with real project
RUN rm src/*
COPY src/ src/
RUN cargo build --release --target-dir /usr/src/codymaze-web/build

# ---
# Second stage (execution)
FROM debian:jessie-slim AS runtime

#RUN apk add --update bash && \
#    rm -rf /var/cache/apk/*

WORKDIR /app
COPY --from=build /usr/src/codymaze-web/build/release/codymaze-web /app/codymaze-web

EXPOSE 8088

# Drop privileges
#USER 1000

CMD ["/app/codymaze-web"]
