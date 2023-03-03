FROM rust:1.67 AS development

WORKDIR /app

COPY ./ ./

RUN apt update && \
    apt upgrade -y && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli && \
    curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g yarn && \
    yarn install

CMD ["/usr/bin/yarn", "dev"]

FROM development AS release-builder

WORKDIR /artifact

RUN yarn release
