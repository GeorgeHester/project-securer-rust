const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    stats: {
        errorDetails: true
    },
    entry: {
        main: [
            path.resolve(__dirname, "src/app.tsx")
        ]
    },
    output: {
        path: path.resolve(__dirname, "build"),
        filename: "[name].[contenthash].js",
        clean: true,
        assetModuleFilename: "static/[name].[contenthash].[ext]"
    },
    module: {
        rules: [
            {
                test: /\.(ts|js)x?$/i,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader",
                        options: {
                            configFile: path.resolve(__dirname, "tsconfig.json")
                        }
                    }
                ]
            }
        ]
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "public/app.html"),
            filename: "index.html",
            chunks: ["main"]
        })
    ],
    resolve: {
        extensions: [".tsx", ".ts", ".js"]
    }
};