FROM rust:1.81.0-alpine

RUN apk add --update \
    alpine-sdk \
    yt-dlp \
    ffmpeg \
    pkgconfig \
    cmake \
    libressl-dev \
    musl-dev \
    openssl \
    libc6-compat

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release && rm -rf src

COPY . .

RUN cargo build --release

CMD ["./target/release/discord_music_bot"]
