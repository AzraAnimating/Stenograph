FROM rust:slim-bookworm AS builder
RUN apt update && apt install -y perl libfindbin-libs-perl build-essential
COPY . /app
WORKDIR /app
RUN ["cargo", "build", "--release" ]

FROM debian:12.9-slim 
WORKDIR /app
COPY --from=builder /app/target/release/stenograph_api /app/api
RUN chmod +x /app/api 

RUN useradd -ms /bin/bash stenograph
USER stenograph
WORKDIR /app

ENTRYPOINT ["./api"]
