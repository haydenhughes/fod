const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

module.exports = {
  entry: './js/index.js',
  output: {
    path: path.resolve(__dirname, 'static'),
    filename: 'js/bundle.js'
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
    },
    {
      test: /\.(woff|woff2|eot|ttf|svg)$/,
      use: [{
        loader: 'file-loader',
        options: {
          name: '[name].[ext]',
          outputPath: 'fonts/'
        }
      }],
    }]
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: 'fodmap.css'
    }),
  ]
};
