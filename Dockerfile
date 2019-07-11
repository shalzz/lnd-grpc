FROM clux/muslrust:1.35.0-nightly as build

RUN apt-get update \
    && apt-get install unzip 

## Install Protobuf 3.0
RUN curl -OL https://github.com/google/protobuf/releases/download/v3.3.0/protoc-3.3.0-linux-x86_64.zip
RUN unzip protoc-3.3.0-linux-x86_64.zip -d protoc3
RUN mv protoc3/bin/* /usr/local/bin/
RUN mv protoc3/include/* /usr/local/include/

COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock

RUN mkdir src && \
    printf "fn main() {}" > src/main.rs && \
    printf "fn main() {}" > build.rs && \
    cargo build --release && \
    rm src/main.rs build.rs
COPY ./ ./

RUN cargo build --release

RUN mkdir -p ./build

RUN cp ./target/x86_64-unknown-linux-musl/release/ambassador-rust-rate-limiter ./build

FROM alpine

COPY --from=build /volume/build/ambassador-rust-rate-limiter /usr/local/bin/

CMD /usr/local/bin/ambassador-rust-rate-limiter 
