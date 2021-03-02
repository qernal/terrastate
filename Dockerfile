FROM clux/muslrust:1.49.0-stable as builder
LABEL org.opencontainers.image.source https://github.com/qernal/terrastate

WORKDIR /usr/src/app
COPY . ./

RUN mkdir /output
RUN cargo build --verbose --release --target-dir /output

FROM scratch
COPY --from=builder /output/x86_64-unknown-linux-musl/release/terrastate /

EXPOSE 3000
ENTRYPOINT ["/terrastate"]