FROM shinsaku0523/nightly-2021-11-17:latest

ARG EXEC_CMD

ENV EXEC_CMD $EXEC_CMD

COPY . .

RUN cargo build --all-targets

CMD cargo $EXEC_CMD
