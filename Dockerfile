FROM debian:12.9-slim 
WORKDIR /app
COPY target/release/stenograph_api /app/api
RUN chmod +x /app/api 

RUN useradd -ms /bin/bash stenograph
USER stenograph
WORKDIR /app

ENTRYPOINT ["./api"]
