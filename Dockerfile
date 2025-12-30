FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.9.0@sha256:c64defb9ed5a91eacb37f96ccc3d4cd72521c4bd18d5442905b95e2226b0e707 AS xx

FROM --platform=$BUILDPLATFORM rust:1.92.0-slim@sha256:0d8bf269f3ab28ddcdc3a182f519d7499daee82ce2faa4008048ae8adf6977e7 AS builder

COPY --from=xx / /

RUN apt-get update && apt-get install -y clang lld

WORKDIR /usr/src/app

ARG TARGETPLATFORM

RUN xx-apt-get update && \
    xx-apt-get install -y \
    gcc \
    g++ \
    libc6-dev \
    pkg-config

COPY . ./

ARG RESTATE_SERVICE_NAME

RUN xx-cargo build --release --bin restate-examples
RUN xx-verify ./target/$(xx-cargo --print-target-triple)/release/restate-examples
RUN cp -r ./target/$(xx-cargo --print-target-triple)/release/restate-examples /usr/local/bin/restate-examples


# FROM alpine:3.23.0@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375
FROM debian:13.2-slim@sha256:91e29de1e4e20f771e97d452c8fa6370716ca4044febbec4838366d459963801

COPY --from=builder /usr/local/bin/restate-examples /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 9080

CMD ["restate-examples"]
