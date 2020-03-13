#!/bin/bash

set -e

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -y
rustup default nightly
yarn webpack --mode production
