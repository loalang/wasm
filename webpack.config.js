const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

const ASSET_PATH = process.env.ASSET_PATH || "/";
const SCRIPTS = ["loac", "loavm"];

module.exports = {
  entry: Object.fromEntries(
    SCRIPTS.map(name => [name, `./${name}/src/${name}.js`])
  ),
  output: {
    filename: "[name].js",
    chunkFilename: "[name].[chunkhash].js",
    publicPath: ASSET_PATH
  },
  plugins: SCRIPTS.map(
    name =>
      new WasmPackPlugin({
        crateDirectory: path.join(__dirname, name),
        outDir: path.join(__dirname, name, "gen"),
        extraArgs: "--no-typescript"
      })
  )
};
