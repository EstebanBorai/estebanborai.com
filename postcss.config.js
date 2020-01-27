const webpack = require('webpack');
const cssNano = require('cssnano');
const postcssImport = require('postcss-import');

module.exports = {
  syntax: 'postcss-scss',
  plugins: [
    require('precss'),
    require('autoprefixer'),
    cssNano({
      preset: 'default',
    }),
    postcssImport({
      addDependencyTo: webpack,
    }),
  ],
};
