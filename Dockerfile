ARG PROJECT
FROM  ekidd/rust-musl-builder as builder

ENV DEBIAN_FRONTEND noninteractive

RUN sudo apt-get update -y && \
		sudo apt-get install -y cmake pkg-config \
			libxcursor-dev libxrandr-dev libxi-dev libx11-xcb-dev \
			libgl1-mesa-dev mesa-utils libgl1-mesa-glx libasound2-dev \
			libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev python3

WORKDIR /home/rust
RUN rustup toolchain install nightly && rustup default nightly && rustup target add x86_64-unknown-linux-musl

COPY alacritty ../alacritty
COPY Cargo.toml Cargo.toml
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo +nightly build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/release/deps/${PROJECT}*

COPY src src

RUN RUSTFLAGS=-Clinker=musl-gcc cargo +nightly build --release --target=x86_64-unknown-linux-musl --verbose

FROM alpine:latest

COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/${PROJECT} /bin/${PROJECT}
COPY config config

CMD [${PROJECT}]