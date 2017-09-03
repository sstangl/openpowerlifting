// vim: set ts=2 sts=2 sw=2 et:
var path = require('path');

module.exports = {
  entry: {
    index: "./scripts/main-index.ts",
    lifters: "./scripts/main-lifters.js",
    meet: "./scripts/main-meet.js",
    meetlist: "./scripts/main-meetlist.js"
  },

  output: {
    path: path.resolve(__dirname, "build/scripts"),
    filename: "[name].bundle.js",
  },

  module: {
    loaders: [
      {
        test: /\.js$/,
        loader: 'babel-loader',
        query: {
          presets: ['es2015'],
          plugins: [require('babel-plugin-minify-constant-folding')]
        }
      },
      {
        test: /\.ts$/,
        // The loader array executes right-to-left.
        loaders: [
          {
            loader: 'babel-loader',
            query: {
              presets: ['es2015'],
              plugins: [require('babel-plugin-minify-constant-folding')]
            }
          },
          {
            loader: 'awesome-typescript-loader',
            options: {
              allowJs: true
            }
          }
        ],
        exclude: /node_modules/,
      }
    ]
  },

  stats: {
    colors: true
  },

  // devtool: 'source-map'
};
