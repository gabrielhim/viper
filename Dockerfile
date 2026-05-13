FROM rust:1.92 AS base
WORKDIR /usr/src/viper
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=base /usr/local/cargo/bin/viper /usr/local/bin/viper
CMD ["viper"]
