FROM rust:latest

# copy source
WORKDIR /usr/src/asma
COPY . .

# copy config
COPY ./doc/asma.toml /etc/asma.toml

# install dependencies
RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  clang      \
  && rm -rf /var/lib/apt/lists/*

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
