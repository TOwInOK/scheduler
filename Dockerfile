FROM clux/muslrust:stable AS planner
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM clux/muslrust:stable AS cacher
RUN cargo install cargo-chef
COPY --from=planner /volume/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json


FROM clux/muslrust:stable AS builder
COPY . .
COPY --from=cacher /volume/target target
# COPY --from=cacher /root/.cargo /root/.cargo
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static:nonroot
# FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder --chown=nonroot:nonroot /volume/target/x86_64-unknown-linux-musl/release/sheduller /app/sheduller
COPY --from=builder --chown=nonroot:nonroot /volume/store /app/store
ARG BOT_TOKEN
ENV BOT_TOKEN=${BOT_TOKEN}
ENTRYPOINT ["/app/sheduller"]

LABEL org.opencontainers.maintainer="TOwInOK <60252419+TOwInOK@users.noreply.github.com>"
LABEL org.opencontainers.version="1.0.0"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/TOwInOK/scheduler"
LABEL org.opencontainers.image.description="Telegram bot for scheduling lessons"
LABEL org.opencontainers.image.documentation="https://github.com/TOwInOK/scheduler/blob/main/README.md"
