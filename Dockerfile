# Cargo Build Stage

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install -qq gcc-arm-linux-gnueabihf

RUN rustup target add armv7-unknown-linux-gnueabihf

WORKDIR /usr/src/rpiwatchdog

COPY . .

RUN RUSTFLAGS=-Clinker=arm-linux-gnueabihf-gcc cargo build --release --target=armv7-unknown-linux-gnueabihf

RUN rm -f target/armv7-unknown-linux-gnueabihf/release/deps/rpiwatchdog*

RUN RUSTFLAGS=-Clinker=arm-linux-gnueabihf-gcc cargo build --release --target=armv7-unknown-linux-gnueabihf


# Final Stage - Copy executable from cargo-build container into lightweight alpine image

FROM alpine:latest

RUN addgroup -g 1000 rpiwatchdog

RUN adduser -D -s /bin/sh -u 1000 -G rpiwatchdog rpiwatchdog

WORKDIR /home/rpiwatchdog/bin/

COPY --from=cargo-build /usr/src/rpiwatchdog/target/armv7-unknown-linux-gnueabihf/release/rpi_watchdog .

RUN chown rpiwatchdog:rpiwatchdog rpi_watchdog

USER rpiwatchdog

CMD ["./rpi_watchdog"]