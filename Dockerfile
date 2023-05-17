FROM redis:7.0 as redis
FROM rust:bullseye as builder

ADD . /build
WORKDIR /build
RUN apt update -qq && apt install -yqq python3 python3-pip clang
RUN make RELEASE=1

FROM debian:bullseye
RUN apt update -qq && apt install -yqq libssl-dev
RUN rm -rf /var/cache/apt/*
COPY --from=redis /usr/local/bin/redis-* /usr/bin/
COPY --from=builder /build/target/release/libredis_state.so /usr/lib/libredis_state.so
EXPOSE 6379
CMD ["redis-server", "--protected-mode", "no", "--loadmodule", "/usr/lib/libredis_state.so"]
