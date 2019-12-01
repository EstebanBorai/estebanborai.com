import React from 'react';
import './nav.scss';
import { NavLink as Link } from 'react-router-dom';
import { useIntl } from 'react-intl';

const Nav = () => {
	const { messages } = useIntl();
	return (
	<nav id="nav">
		<ol>
			<li>
				<Link to="/">{messages.nav.home}</Link>
			</li>
			<li>
				<Link to="/repositories">{messages.nav.repositories}</Link>
			</li>
		</ol>
	</nav>
	);
};

export default Nav;
