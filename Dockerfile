# Build Stage
FROM rust:1.52.0 AS builder
WORKDIR /src/
COPY . /src
RUN cargo build --release

# Bundle Stage
FROM rust:1.52.0
COPY --from=builder /src/target/release/my-imap .
RUN mkdir -p /var/run/my-imap && chown 1000:1000 /var/run/my-imap
USER 1000
HEALTHCHECK --interval=60s --timeout=5s --start-period=120s --retries=3 \
  CMD grep -q '^ok$' /var/run/my-imap/status || { cat /var/run/my-imap/status 2>/dev/null; exit 1; }
CMD ["/my-imap"]
