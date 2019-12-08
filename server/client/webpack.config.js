// vim: set ts=2 sts=2 sw=2 et:
const path = require('path');

var babelOptions = {
  "presets": [
    ["@babel/preset-env", {
      "modules": false
    }]
  ]
};

module.exports = {
  entry: {
    main: "./scripts/main.ts",
    checker: "./scripts/checker/checker.ts"
  },

  output: {
    path: path.resolve(__dirname, "build/scripts"),
    filename: "[name].js",
  },

  mode: "production",

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            options: babelOptions
          }
        ]
      },
      {
        test: /\.ts(x?)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            options: babelOptions
          },
          {
            loader: 'ts-loader'
          }
        ]
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
