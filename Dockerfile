FROM clux/muslrust:stable as builder
WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release

FROM gcr.io/distroless/static-debian11:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/teamsykmelding-pik-2 /app/teamsykmelding-pik-2
EXPOSE 8080
CMD ["/app/teamsykmelding-pik-2"]