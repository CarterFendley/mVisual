// webpack.config.js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = [
  {
    mode: 'development',
    entry: './src/main.ts',
    target: 'electron-main',
    module: {
      rules: [{
        test: /\.ts$/,
        include: /src/,
        use: [{ loader: 'ts-loader' }]
      }]
    },
    output: {
      path: __dirname + '/dist',
      filename: 'electron.js'
    }
  },
  {
    mode: 'development',
    entry: './src/preload.ts',
    target: 'electron-preload',
    devtool: 'source-map',
    module: { rules: [{
      test: /\.ts$/,
      include: /src/,
      use: [{ loader: 'ts-loader' }]
    }] },
    output: {
      path: __dirname + '/dist',
      filename: 'preload.js'
    }
  },
  {
    mode: 'development',
    entry: './src/renderer.ts',
    target: 'electron-renderer',
    devtool: 'source-map',
    module: { rules: [{
      test: /\.ts$/,
      include: /src/,
      use: [{ loader: 'ts-loader' }]
    }] },
    output: {
      path: __dirname + '/dist',
      filename: 'renderer.js'
    },
    experiments: {
      asyncWebAssembly: true,
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: './src/index.html'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, 'rs'),
        withTypeScript: true,
        //outDir: path.resolve(__dirname, 'pkg')
        extraArgs: `--out-dir ${path.resolve(__dirname, 'pkg')}`
      })
    ]
  }
];