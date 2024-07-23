FROM rust:1.78.0 AS build

# Install protobuf compiler
RUN USER=root apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app

COPY . .

# Build your project
RUN cargo build --release

# Use a minimal Ubuntu image for the final image
FROM gcr.io/distroless/cc-debian12
FROM ubuntu:20.04

COPY --from=build /app/target/release/microservice-rcpp /

EXPOSE 50051

CMD ["./microservice-rcpp"]
