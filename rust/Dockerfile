FROM --platform=linux/amd64 rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/src src
RUN cargo build --release

FROM --platform=linux/amd64 scratch
COPY --from=build /src/target/release/checksum /checksum
COPY data /data
EXPOSE 4000
ENTRYPOINT ["/checksum"]
