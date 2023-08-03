# Build stage
FROM rust:1.70-buster as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/teamsykmelding-pik-2 .
EXPOSE 8080

CMD ["./teamsykmelding-pik-2"]
