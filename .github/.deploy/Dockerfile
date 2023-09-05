FROM ubuntu:latest

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

WORKDIR /enstate

COPY target/release/enstate /enstate/enstate

CMD ["/enstate/enstate"]
