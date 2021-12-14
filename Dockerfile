FROM ubuntu:16.04

WORKDIR /app

# rust path
ENV PATH=$PATH:/root/.cargo/bin

RUN apt-get update -y &&\
    apt-get install build-essential curl wget -y

# enable rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2021-11-17

COPY . .

RUN cargo test --no-run

CMD cargo test
