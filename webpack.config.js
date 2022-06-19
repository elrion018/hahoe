const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProduction = process.env.NODE_ENV == "production";

const stylesHandler = isProduction
  ? MiniCssExtractPlugin.loader
  : "style-loader";

const config = {
  entry: path.resolve(__dirname, 'public/index.ts'),
  output: {
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    open: true,
    host: "localhost",
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'public/index.html'),
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'gui'),
      outDir: "pkg",
    }),
  ],
  module: {
    rules: [
      {
        test: /\.ts$/i,
        loader: "ts-loader",
        exclude: ["/node_modules", /\.wasm$/],
      },
      {
        test: /\.css$/i,
        use: [stylesHandler, "css-loader"],
        exclude: [ /\.wasm$/],
      },
      {
        test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/i,
        type: "asset",
        exclude: [ /\.wasm$/],
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js", '.wasm'],
  },
};

module.exports = (_, env) => {
  if (isProduction) {
    config.mode = "production";

    config.plugins.push(new MiniCssExtractPlugin());
  } else {
    config.mode = "development";
  }
  return config;
};