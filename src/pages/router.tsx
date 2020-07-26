import React from "react";
import {
  Switch,
  Route
} from "react-router-dom";
import Home from './Home';
import About from './About';

function Router() {
  return (
    <Switch>
      <Route exact path="/">
        <Home />
      </Route>
      <Route exact path="/about">
        <About />
      </Route>
    </Switch>
  );
}

export default Router;
