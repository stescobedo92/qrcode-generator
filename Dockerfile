# Use the official Rust image from Docker Hub as the base image
FROM rust:latest AS builder

WORKDIR /usr/src/qrcode-generator

# Copy the Cargo.lock and Cargo.toml files to the working directory
COPY Cargo.lock Cargo.toml ./

# Create the source directory
RUN mkdir src

# Copy the source code to the working directory
COPY src/ ./src/

# Build the application
RUN cargo build --release --bin bncfqr

# Use a lightweight Alpine Linux image as the final base image
FROM alpine:latest

# Set the working directory in the container
WORKDIR /usr/src/qrcode-generator

# Copy the built executable from the builder stage to the final image
COPY --from=builder /usr/src/qrcode-generator/target/release/bncfqr ./

# Run the executable
CMD ["bncfqr"]