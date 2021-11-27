FROM shinsaku0523/nightly-2021-11-17:latest

COPY . .

RUN cargo test --no-run

CMD cargo test
