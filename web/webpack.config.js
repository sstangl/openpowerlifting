var path = require('path');

module.exports = {
  entry: {
    index: "./scripts/main-index.js",
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
          plugins: ['minify-constant-folding']
        }
      }
    ]
  },

  stats: {
    colors: true
  },

  // devtool: 'source-map'
};
