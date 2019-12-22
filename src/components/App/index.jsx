import React from 'react';
import './app.scss';
import {
	BrowserRouter as Router,
	Switch,
	Route,
} from 'react-router-dom';
import Loader from 'components/Loader';
import Home from 'sections/Home';
import About from 'sections/About';
import Projects from 'sections/Projects';

const App = () => (
	<Router>
		<Home />
		<About />
		<Projects />
	</Router>
);

const withSuspense = () => (
	<React.Suspense fallback={<Loader />}>
		<App />
	</React.Suspense>
);

export default withSuspense;
