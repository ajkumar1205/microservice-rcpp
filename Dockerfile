# Use the latest Ubuntu image as the base
FROM ubuntu:latest as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    gcc \
    g++ \
    protobuf-compiler \
    libprotobuf-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Use a minimal Ubuntu image for the final image
FROM ubuntu:latest

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/app /

# Expose the port (if needed)
EXPOSE 50051

# Set the entrypoint to run the binary
ENTRYPOINT ["/app"]