# Use a rust image with the latest version of Rust installed
FROM rust:1.71.1-bullseye

# Set the working directory in the container
WORKDIR /app

# Copy the source code
COPY . .

# Build the Rust application
RUN cargo build --release

# Expose port 8080 in the container
EXPOSE 8080

# Specify the command to run when the container starts
CMD ["./target/release/teamsykmelding-pik-2"]
