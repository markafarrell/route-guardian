FROM rust:1.62.0 as builder
WORKDIR /usr/src/route-guardian
COPY . .
RUN cargo install --path .

FROM alpine
RUN apk add iproute2 libgcc gcompat
COPY --from=builder /usr/local/cargo/bin/route-guardian /usr/local/bin/route-guardian
CMD ["route-guardian"]
