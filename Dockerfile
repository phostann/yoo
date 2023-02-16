FROM rust:1.67.1 as builder
WORKDIR /usr/src/yoo
COPY . .

ENV CARGO_REGISTRY_DEFAULT=git://mirrors.ustc.edu.cn/crates.io-index
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
RUN cd /usr/lib && ln -s libssl.so libssl.so.1.1
COPY --from=builder /usr/src/yoo/target/release/yoo /usr/local/bin/yoo
COPY --from=builder /usr/src/yoo/.env /usr/local/bin/.env
EXPOSE 9999

WORKDIR /usr/local/bin
CMD ["yoo"]
