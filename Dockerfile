# build stage 
FROM rust:1.81.0-alpine AS builder

RUN set -eux \
  && apk update \
  && apk add ca-certificates gcc alpine-sdk \
  && apk add openssl-dev openssl-libs-static musl-dev \
  && rustup target add x86_64-unknown-linux-musl \
  && update-ca-certificates

ENV USER=myapp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /build
COPY . .
RUN --mount=type=ssh cargo build --target=x86_64-unknown-linux-musl --release

# runtime stage
FROM alpine:latest

RUN apk update

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/photojournalism .
COPY --from=builder /build/feeds.txt .
COPY --from=builder /build/static /app/static

USER myapp:myapp

EXPOSE 9000

CMD ["/app/photojournalism"]
