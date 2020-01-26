const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const DefinePlugin = require('webpack').DefinePlugin;
const HotModuleReplacementPlugin = require('webpack').HotModuleReplacementPlugin;

module.exports = (env, args) => {
  const config = {};
  const isDevelopment = env === 'development';

  if (isDevelopment) {
    console.log('Running on Development Mode');
  }

  config.devtool = isDevelopment && 'source-map';

  // Entry File
  config.entry = './src/index.tsx';

  // Bundle target file
  config.output = {
    filename: 'index.js',
  };

  // Modules configuration
  config.module = {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: 'style.css',
            }
          },
          {
            loader: 'extract-loader'
          },
          {
            loader: 'css-loader?-url'
          },
          {
            loader: 'postcss-loader'
          },
          {
            loader: 'sass-loader'
          }
        ],
      },
    ]
  };

  // Webpack Dev Server
  config.devServer = {
    contentBase: path.join(__dirname, 'src'),
    hot: true,
    inline: true,
    compress: true,
    port: 8080,
    historyApiFallback: true,
    watchOptions: {
      ignored: /node_modules/,
      aggregateTimeout: 1500,
    },
    stats: {
      colors: true,
      hash: false,
      version: false,
      timings: false,
      assets: false,
      chunks: false,
      modules: false,
      reasons: false,
      children: false,
      source: false,
      errors: true,
      errorDetails: true,
      warnings: true,
      publicPath: false
    }
  };

  config.plugins = [
    new HtmlWebpackPlugin({
      title: 'Esteban Borai | GitHub Page',
      template: './public/index.ejs',
    }),
    new HotModuleReplacementPlugin(),
  ];

  config.resolve = {
    extensions: ['.ts', '.tsx', '.js'],
    alias: {
      components: path.resolve(__dirname, 'src/components'),
    }
  };

  return config;
};
