# Use an official Rust runtime as a parent image
FROM rust:1.76.0-slim-buster as builder

# Set the working directory in the container to /app
WORKDIR /app

# Install OpenSSL and ca-certificates
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# Copy your source tree
COPY ./src ./src

# This build step will cache your dependencies
RUN cargo build --release

# Build for release.
RUN rm ./target/release/deps/daily_notes_reminder*
RUN cargo build --release

# Our second stage, that only gets the built binary
FROM debian:buster-slim

# Install OpenSSL and ca-certificates
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates

# copy the build artifact from the build stage
COPY --from=builder /app/target/release/daily_notes_reminder /usr/local/bin

# Run the binary.
CMD ["daily_notes_reminder"]