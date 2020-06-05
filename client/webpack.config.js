const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
  entry: {
    index: './js/index.js'
  },
  output: {
    path: dist,
    filename: '[name].js'
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [{
      test: /\.scss$/,
      use: [
        MiniCssExtractPlugin.loader,
        {
          loader: 'css-loader'
        },
        {
          loader: 'sass-loader',
          options: {
            sourceMap: true,
            // options...
          }
        }
      ]
    }]
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, 'static')
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new MiniCssExtractPlugin({
      filename: 'styles.css'
    }),
  ]
};
