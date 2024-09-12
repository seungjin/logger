FROM rust:latest as builder
WORKDIR /work
ADD ./ /work/
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /work/target/release/logger .
EXPOSE 8080
CMD ["logger"]

