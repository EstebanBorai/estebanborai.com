import React from 'react';
import './panel.scss';

const Panel = ({ name }) => (
	<section className="panel">
		<header className="panel-header">{name}</header>
	</section>
)

export default Panel;
