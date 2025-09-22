FROM clux/muslrust:stable AS builder
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static:nonroot
# FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder --chown=nonroot:nonroot /volume/target/aarch64-unknown-linux-musl/release/sheduller /app/sheduller
COPY --from=builder --chown=nonroot:nonroot /volume/store /app/store
ENTRYPOINT ["/app/sheduller"]
