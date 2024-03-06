# Use the official Rust base image to build the binary
FROM rust:latest AS build

WORKDIR /app

# Copy only the manifest files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build a dummy project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the source code and build the actual binary
COPY src ./src
RUN cargo build --release

# Create a minimal image with just the binary
FROM debian:buster-slim

# Copy the binary from the build stage to the final image
COPY --from=build /app/target/release/openstackrs /usr/local/bin/openstackrs

# Set the entry point for the container
ENTRYPOINT ["/usr/local/bin/openstackrs"]
 
