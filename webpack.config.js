const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

const ASSET_PATH = process.env.ASSET_PATH || "/";

const sharedWasmPackConfig = {
  extraArgs: "--no-typescript",
};

module.exports = {
  entry: {
    loac: "./loac/src/loac.js",
    loavm: "./loavm/src/loavm.js"
  },
  output: {
    filename: "[name].js",
    chunkFilename: "[name].[chunkhash].js",
    publicPath: ASSET_PATH
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, "loac"),
      outDir: path.join(__dirname, "loac", "gen"),
      ...sharedWasmPackConfig
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, "loavm"),
      outDir: path.join(__dirname, "loavm", "gen"),
      ...sharedWasmPackConfig
    })
  ]
};
