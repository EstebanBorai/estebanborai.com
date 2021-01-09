module.exports = {
  mount: {
    public: '/',
    src: '/_dist_',
  },
  optimize: {
    bundle: true,
    minify: true,
    target: 'es2018',
  },
  plugins: [
    '@snowpack/plugin-react-refresh',
    '@snowpack/plugin-dotenv',
    '@snowpack/plugin-typescript',
  ],
  install: [
    /* ... */
  ],
  installOptions: {
    /* ... */
  },
  devOptions: {
    open: 'none',
  },
  buildOptions: {
    /* ... */
  },
  proxy: {
    /* ... */
  },
  alias: {
    '@estebanborai.com': './src'
  },
};
