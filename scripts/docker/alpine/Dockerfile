FROM alpine:edge AS builder

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  build-base \
  cargo \
  cmake \
  eudev-dev \
  linux-headers \
  perl \
  rust

WORKDIR /tetsy
COPY . /tetsy
RUN cargo build --release --target x86_64-alpine-linux-musl --verbose
RUN strip target/x86_64-alpine-linux-musl/release/tetsy


FROM alpine:edge

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  libstdc++ \
  eudev-libs \
  libgcc

RUN addgroup -g 1000 tetsy \
  && adduser -u 1000 -G tetsy -s /bin/sh -D tetsy

USER tetsy

EXPOSE 8080 8545 8180

WORKDIR /home/tetsy

RUN mkdir -p /home/tetsy/.local/share/io.tetsy.vapory/
COPY --chown=tetsy:tetsy --from=builder /tetsy/target/x86_64-alpine-linux-musl/release/tetsy ./

ENTRYPOINT ["./tetsy"]
