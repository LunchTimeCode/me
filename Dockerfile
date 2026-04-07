# Build stage
FROM rust:1.94-slim as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

COPY Rocket.toml Rocket.toml
# Build your program for release
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/me /usr/local/bin/
    
ENV ROCKET_PORT=80 
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 80
# Set the startup command
CMD ["me"]
