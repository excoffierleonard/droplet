FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/release/droplet .
EXPOSE 8080
VOLUME ["/app/share"]
CMD ["./droplet"]