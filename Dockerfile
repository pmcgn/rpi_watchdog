FROM rust:latest

WORKDIR /usr/src/rpi_watchdog

COPY . .

RUN cargo build --release --target=arm-unknown-linux-gnueabihf

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rpi_watchdog"]