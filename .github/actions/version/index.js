const { readFileSync } = require("fs");

const cargoManifest = readFileSync("loa/Cargo.toml", "utf-8");

const version = cargoManifest.match(/version = "(.*?)"/)[1];

console.log(`::set-output name=version::${version}`)
