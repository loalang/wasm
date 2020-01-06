const WorkerPlugin = require("worker-plugin");

module.exports = {
    output: {
        globalObject: "self",
        chunkFilename: "[contenthash].[name].js"
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
