#!/bin/bash

set -e

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
rustup default nightly

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

bash -c "$1"
