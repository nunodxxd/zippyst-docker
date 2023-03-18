FROM rust:latest AS builder

WORKDIR /zippyst

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /zippyst/target/release/zippyst-cmd /zippyst-cmd

USER nonroot

ENTRYPOINT ["/zippyst-cmd"]

CMD ["/bin/sh", "-c", "/zippyst/zippyst-cmd \"$@\"", "sh"]
