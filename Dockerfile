FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libopus-dev \
    pkg-config \
    build-essential

WORKDIR /build
COPY entrypoint.sh entrypoint.sh
COPY Cargo.lock Cargo.toml ./
COPY src src
COPY migrations migrations
RUN export RUST_MIN_STACK=33554432 && cargo clean &&  cargo build --locked --release --workspace

FROM debian:bullseye-slim AS final
WORKDIR /app


## install deps
RUN apt-get update && apt-get install -y \
    libopus-dev \
    build-essential \
    autoconf \
    wget \
    automake \
    ffmpeg \
    libtool \
    m4 

## download yt-dlp
RUN wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux

COPY --from=build /build/target/release/chahaein /app
COPY --from=build /build/target/release/migrations /app
COPY --from=build /build/entrypoint.sh /app

USER root
RUN chmod +x entrypoint.sh
RUN chmod +x yt-dlp_linux
RUN mv /app/yt-dlp_linux /usr/local/bin/yt-dlp
RUN ls -lsa

RUN addgroup --system --gid 1001 chahaein
RUN adduser --system --uid 1001 chahaein

USER chahaein

CMD ["./entrypoint.sh"]
