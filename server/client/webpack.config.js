// vim: set ts=2 sts=2 sw=2 et:
const path = require('path');
const webpack = require('webpack')

const BabelMinifyConstantFolding = require('babel-plugin-minify-constant-folding');

module.exports = {
  entry: {
    rankings: "./scripts/rankings.ts"
  },

  output: {
    path: path.resolve(__dirname, "build/scripts"),
    filename: "[name].js",
  },

  module: {
    loaders: [
      {
        test: /\.js$/,
        loader: 'babel-loader',
        query: {
          presets: ['env'],
          plugins: [BabelMinifyConstantFolding]
        }
      },
      {
        test: /\.ts$/,
        // The loader array executes right-to-left.
        loaders: [
          {
            loader: 'babel-loader',
            query: {
              presets: ['env'],
              plugins: [BabelMinifyConstantFolding]
            }
          },
          {
            loader: 'awesome-typescript-loader',
            options: {
              // Setting to "true" causes the loader to consult the build/
              // directory, where it tries to load the openpowerlifting.js
              // and OOMs.
              allowJs: false
            }
          }
        ],
        exclude: /node_modules/,
      }
    ]
  },

  // Allow use of "include" statements with TypeScript files.
  resolve: {
    extensions: ['.ts', '.tsx', '.js']
  },

  plugins: [
    // FIXME: Disable the common bundle for the moment.
    // new webpack.optimize.CommonsChunkPlugin({
    //   name: 'common'
    // })
  ],

  stats: {
    colors: true
  },

  // devtool: 'source-map'
};
