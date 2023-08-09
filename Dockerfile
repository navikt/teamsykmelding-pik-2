# Build stage
FROM rust:1.71.1-buster as builder

RUN apt-get update \
    && apt-get install -y pkg-config make g++ libssl-dev openssl libudev-dev zlib1g-dev lib64z1 
    
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM gcr.io/distroless/cc-debian11

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/teamsykmelding-pik-2 .
EXPOSE 8080

CMD ["./teamsykmelding-pik-2"]
