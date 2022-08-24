# Source: https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/

FROM rust as builder
WORKDIR /src/WeirdRust
COPY . .
RUN cargo install --path . --color always

FROM ubuntu as runtime
COPY --from=builder /usr/local/cargo/bin/weird_rust /usr/local/bin/weird_rust
CMD ["weird_rust"]