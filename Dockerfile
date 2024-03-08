FROM rust:latest AS build

WORKDIR /app


COPY Cargo.toml Cargo.lock .
COPY src src
RUN cargo build --release

RUN cargo install --path .

# Final image
FROM debian:stable-slim AS release

# Install openssl libraries
RUN apt update \
    && apt install -y libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN addgroup app \
    && adduser --ingroup app --no-create-home app \
    && chown -R app /app

COPY --from=build --chown=app:app /usr/local/cargo/bin/openstackrs .

USER app

ENTRYPOINT ["/app/openstackrs"]
 
