import * as React from 'react';
import ReactDOM from 'react-dom';
import './style.scss';
import App from 'components/App';

if (module.hot) {
  module.hot.accept();
}

ReactDOM.render(<App />, document.getElementById('root'));
