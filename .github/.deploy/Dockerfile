####################################################################################################
## Base image
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

WORKDIR /enstate

COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

RUN cargo build --release

RUN rm -rf ./src

COPY build.rs build.rs
COPY src src
COPY .git .git

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

WORKDIR /enstate

# Copy our build
COPY --from=builder /enstate/target/release/enstate /enstate/enstate

CMD ["/enstate/enstate"]
