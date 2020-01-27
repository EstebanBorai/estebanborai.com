module.exports = {
  syntax: 'postcss-scss',
  plugins: [
    require('precss'),
    require('autoprefixer'),
    require('cssnano')({
      preset: 'default',
    })
  ],
};
