FROM rust:latest

# copy source
WORKDIR /usr/src/asma
COPY . .

# copy config
COPY ./doc/asma.prod.toml /etc/asma.toml

# install dependencies
RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  clang \
  && rm -rf /var/lib/apt/lists/*

# install protoc
ENV PROTOC_ZIP=protoc-23.0-linux-x86_64.zip 
RUN apt-get update && apt-get install -y unzip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v23.0/$PROTOC_ZIP \
    && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
    && unzip -o $PROTOC_ZIP -d /usr/local 'include/*' \ 
    && rm -f $PROTOC_ZIP

# environment variables
ENV ASMA_CONFIG=/etc/asma.toml
ENV ASMA_LOG=info
ENV SQLX_OFFLINE=true

# build
RUN cargo build --release

# run
CMD ["./target/release/driver_web_runner"]

# ports
EXPOSE 8080
EXPOSE 8888
