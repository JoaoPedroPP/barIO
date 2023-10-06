FROM --platform=linux/amd64 rust:slim as cargo-build

WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install build-essential libssl-dev libsasl2-dev openssl cmake -y

COPY . .

RUN cargo build --release

FROM --platform=linux/amd64 debian:latest

RUN apt-get update
RUN apt-get install ca-certificates libssl-dev libsasl2-dev openssl -y

WORKDIR /src/app

COPY .env .

COPY --from=cargo-build /usr/src/app/target/release/barIO .

ENV RUST_LOG=info

# Run the application
CMD ["./barIO"]