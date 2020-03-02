/* eslint-disable */
const webpack = require('webpack');
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HotModuleReplacementPlugin = require('webpack').HotModuleReplacementPlugin;
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const MODE = {
  DEV: 'development',
  PROD: 'production'
}

module.exports = (env, args) => {
  const isDev = args.mode === MODE.DEV;
  const isProd = args.mode === MODE.PROD;

  const config = {
    module: {}
  };

  config.entry = './src/index.tsx';

  config.output = {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].[hash].js'
  }

  if (isDev) {
    config.devtool = 'source-maps';
  }

  config.module.rules = [
    {
      test: /\.tsx?$/,
      use: [
        {
          loader: 'ts-loader',
          options: {
            transpileOnly: true
          }
        }
      ]
    },
    {
      test: /\.css$/,
      use:  [
        MiniCssExtractPlugin.loader,
        'css-loader'
      ]
    },
    {
      test: /\.scss$/,
      use: isDev ? [
        'style-loader',
        'css-loader',
        'sass-loader'
      ] : [
        'style-loader',
        MiniCssExtractPlugin.loader,
        'css-loader',
        'sass-loader'
      ]
    },
    {
      test: /\.(png|jpe?g|gif|svg)$/i,
      use: [
        {
          loader: 'file-loader',
          options: {
            name: 'assets/[name].[ext]',
          }
        }
      ]
    },
    {
      test: /\.(woff|woff2|eot|ttf|otf)$/,
      loader: 'file-loader',
      options: {
        name: 'assets/[name].[ext]'
      }
    }
  ];

  config.plugins = [
    new HtmlWebpackPlugin({
      title: 'Esteban Borai | GitHub Page',
      template: 'public/index.ejs',
      // favicon: 'public/favicon.png',
    }),
    new HotModuleReplacementPlugin()
  ];
  
  if (isDev) {
    config.devServer = {
      contentBase: path.join(__dirname, 'dist'),
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
  }

  if (isProd) {
    config.plugins.push(new CleanWebpackPlugin());
    config.plugins.push(new MiniCssExtractPlugin({
      filename: 'style.[hash].css'
    }));
  }

  config.resolve = {
    extensions: ['.js', '.ts', '.tsx'],
    alias: {
      '@types': path.resolve(__dirname, 'src/@types'),
      components: path.resolve(__dirname, 'src/components'),
      context: path.resolve(__dirname, 'src/context')
    }
  }

  return config;
}
