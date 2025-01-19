# Use official Rust image to build the service
FROM rust:1.70-alpine as builder

# Set up debug
RUN set -xe

# Install dependencies
RUN apk add --no-cache libssl3 bash gettext musl-dev

# Set working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application (release build)
RUN cargo build --release

# Use a smaller runtime image to reduce size
FROM alpine:latest

# Install required runtime dependencies
RUN apk add --no-cache libssl3 bash gettext

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/game .

# ENV
ENV GAME_SERVICE_PORT=8002

# Expose necessary ports (PostgreSQL, microservice port)
EXPOSE 5432
EXPOSE ${GAME_SERVICE_PORT}

# Run the binary
CMD ["./player"]
