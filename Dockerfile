# Build Stage
FROM rust:1.52.0 AS builder
WORKDIR /src/
COPY . /src
RUN cargo build --release

# Bundle Stage
FROM rust:1.52.0
COPY --from=builder /src/target/release/my-imap .
USER 1000
CMD ["/my-imap"]