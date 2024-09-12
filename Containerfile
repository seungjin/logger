FROM rust:latest as builder
WORKDIR /work
ADD ./ /work/
RUN apt update; apt install ca-certificates;
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/sbin/update-ca-certificates /usr/sbin/update-ca-certificates
RUN update-ca-certificates
COPY --from=builder /work/target/release/logger .
EXPOSE 8080
CMD ["./logger"]

