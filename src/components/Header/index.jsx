import React from 'react';
import './header.scss';
import Nav from './Nav';

const Header = () => (
	<header id="app-header">
		{/* <h1>Hi, I&apos;m Esteban Borai</h1> */}
		<h1>Hello World.</h1>
		<em>What a "cliché".. (test)</em>
		<Nav />
	</header>
);

export default Header;
