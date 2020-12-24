FROM ekidd/rust-musl-builder:nightly-2020-11-19 AS build-env

COPY rpkg-api/src src
COPY rpkg-api/Cargo.lock Cargo.lock
COPY rpkg-api/Cargo.toml Cargo.toml
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
WORKDIR /opt/rpkg-api/
COPY --from=build-env /home/rust/src/target/x86_64-unknown-linux-musl/release/rpkg-api .
# COPY rpkg-api/data/pkg.db data/pkg.db
CMD ["/opt/rpkg-api/rpkg-api"]