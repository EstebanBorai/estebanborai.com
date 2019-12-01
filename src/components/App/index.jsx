import React from 'react';
import './app.scss';
import {
	BrowserRouter as Router,
	Switch,
	Route,
} from 'react-router-dom';
import Home from 'pages/Home';
import Repositories from 'pages/Repositories';
import Loader from 'components/Loader';
import Sidebar from '../Sidebar';
import Nav from '../Nav';

const App = () => (
	<Router>
		<div id="app-main">
			<Sidebar />
			<Nav />
			<main id="app-context">
				<div>
					<Switch>
						<Route exact path="/">
							<Home />
						</Route>
						<Route path="/repositories">
							<Repositories />
						</Route>
					</Switch>
				</div>
			</main>
		</div>
	</Router>
);

const withSuspense = () => (
	<React.Suspense fallback={<Loader />}>
		<App />
	</React.Suspense>
);

export default withSuspense;
