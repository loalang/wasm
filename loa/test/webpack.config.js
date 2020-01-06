const WorkerPlugin = require("worker-plugin");

module.exports = {
    output: {
        filename: "[name].js",
        chunkFilename: "[name].[chunkhash].js",
        globalObject: "self"
    },
    plugins: [
        new WorkerPlugin(),
    ],
    devServer: {
        contentBase: "public"
    },
    optimization: {
        splitChunks: {
        }
    }
};
