import React from 'react';
import './app.scss';
import {
  BrowserRouter as Router,
  Switch,
  Route,
} from 'react-router-dom';
import Header from 'components/Header';
import Home from 'pages/Home';
import About from 'pages/About';

const App = () => (
  <Router>
    <div id="app-main">
      <Header />
      <main>
        <Switch>
          <Route exact path="/">
            <Home />
          </Route>
          <Route path="/about">
            <About />
          </Route>
        </Switch>
      </main>
    </div>
  </Router>
);

export default App;
