FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY xargo.sh /
RUN bash /xargo.sh

COPY lld.sh /
RUN bash /lld.sh

COPY qemu.sh /
RUN bash /qemu.sh 2.9.0 aarch64

COPY aarch64-unknown-linux-steed.json /json

ENV RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
