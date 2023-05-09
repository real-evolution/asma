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
  mold       \
  && rm -rf /var/lib/apt/lists/*

# environment variables
ENV ASMA_CONFIG=/etc/asma.toml
ENV ASMA_LOG=info

# run
CMD ["cargo", "run", "--release"]

# ports
EXPOSE 8080
EXPOSE 8888
