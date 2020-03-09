ARG PROJECT
# FROM  rust:latest as builder

# ENV DEBIAN_FRONTEND noninteractive

# RUN apt-get update -y && \
# 		apt-get install -y cmake pkg-config \
# 			libxcursor-dev libxrandr-dev libxi-dev libx11-xcb-dev \
# 			libgl1-mesa-dev mesa-utils libgl1-mesa-glx libasound2-dev \
# 			libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev python3

# WORKDIR /home/rust

# COPY alacritty alacritty
FROM alacritty:latest
WORKDIR /home/rust
COPY Cargo.toml Cargo.toml
RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN pwd
RUN cargo build --release
RUN rm -f target/release/deps/${PROJECT}*

COPY src src

RUN cargo install --path .

ENV BIN ${PROJECT}

ENV RUST_BACKTRACE 1

CMD ["alloy"]