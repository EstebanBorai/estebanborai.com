window.addEventListener('load', () => {
  try {
    hljs.highlightAll();
    console.info('Initialized Highlight.js with success!');
  } catch (err) {
    console.error('Failed to initialize Highlight.js');
    console.error(err);
  }
});
