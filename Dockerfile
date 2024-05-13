FROM gcr.io/distroless/static-debian11:nonroot
WORKDIR /app
COPY target/release/build/target/x86_64-unknown-linux-musl/release/teamsykmelding-pik-2 /app/teamsykmelding-pik-2
EXPOSE 8080
CMD ["/app/teamsykmelding-pik-2"]