# Use official Rust image to build the service
FROM rust:1.70-alpine AS builder

# Set up working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application (release build)
RUN cargo build --release

# Use a smaller runtime image to reduce size
FROM alpine:latest

# Set the working directory for the runtime container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/player .

# Set environment variables (optional, you can override these at runtime)
ENV PLAYER_SERVICE_PORT=8001

# Export port
EXPOSE ${PLAYER_SERVICE_PORT}

# Command to run the binary
CMD ["./player"]
