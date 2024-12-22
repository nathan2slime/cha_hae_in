FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential

WORKDIR /build
COPY entrypoint.sh entrypoint.sh
COPY Cargo.lock Cargo.toml ./
COPY src src
COPY migrations migrations
RUN cargo build --locked --release --workspace


FROM debian:bullseye-slim AS final

WORKDIR /app
COPY --from=build /build/target/release/chahaein /app
COPY --from=build /build/target/release/migrations /app
COPY --from=build /build/entrypoint.sh /app

USER root
RUN chmod +x entrypoint.sh

RUN addgroup --system --gid 1001 chahaein
RUN adduser --system --uid 1001 chahaein

USER chahaein

CMD ["./entrypoint.sh"]
