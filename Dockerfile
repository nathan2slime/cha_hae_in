FROM rust:1.83.0-slim-bullseye AS build

WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY src src
RUN cargo build --locked --release


FROM debian:bullseye-slim AS final

WORKDIR /app
COPY --from=build /build/target/release/chahaein /app

CMD "./chahaein"
