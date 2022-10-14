FROM rustlang/rust:nightly-slim AS builder

WORKDIR app
COPY . .
RUN cargo build --release --bin hello-gitops-rust

FROM ubuntu:22.10 AS runtime
ARG USERNAME=gitops
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
USER $USERNAME
WORKDIR /home/$USERNAME
COPY --from=builder /app/target/release/hello-gitops-rust /usr/local/bin
COPY --from=builder /app/Rocket.toml /home/$USERNAME
EXPOSE 8080
CMD ["hello-gitops-rust"]
