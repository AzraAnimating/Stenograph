FROM rust:slim-bookworm AS builder
RUN apt update && apt install -y perl libfindbin-libs-perl build-essential
COPY . /app
WORKDIR /app
RUN ["cargo", "build", "--release" ]

FROM debian:12.9-slim 
RUN useradd -ms /bin/bash stenograph
WORKDIR /app
COPY --from=builder /app/target/release /app

WORKDIR /app
RUN chmod +x /app/stenograph_api && chown -R stenograph /app
USER stenograph

ENTRYPOINT ["/app/stenograph_api"]
