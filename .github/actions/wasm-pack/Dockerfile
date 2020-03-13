FROM node:12

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y

ENV PATH "$PATH:/root/.cargo/env"

RUN rustup default nightly

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
