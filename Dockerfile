# Build stage
FROM rust:1.71.1-buster as builder

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

RUN ln -s libssl.so libssl.so.1.1

COPY --from=builder /app/target/release/teamsykmelding-pik-2 .
EXPOSE 8080

CMD ["./teamsykmelding-pik-2"]
