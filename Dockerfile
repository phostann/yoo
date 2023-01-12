FROM rust:1.66 as builder
WORKDIR /usr/src/yoo
COPY . .

ENV CARGO_REGISTRY_DEFAULT=git://mirrors.ustc.edu.cn/crates.io-index
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/yoo/target/release/yoo /usr/local/bin/yoo
COPY --from=builder /usr/src/yoo/.env /usr/local/bin/.env
EXPOSE 9999

WORKDIR /usr/local/bin
CMD ["yoo"]