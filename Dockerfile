FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/travel_time ./travel_time

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3


# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/travel_time /app/travel_time

EXPOSE 3001

# Run the app
CMD ./travel_time
