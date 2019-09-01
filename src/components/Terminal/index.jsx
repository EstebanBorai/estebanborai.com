import React from 'react';
import './terminal.scss';
import Panel from 'components/Panel';

const defaultPanel = {
	name: 'Menu'
};

const Terminal = () => {
	const activePanel = React.useState(1);
	const panels = React.useState([
		defaultPanel
	]);

	return (
		<main id="terminal" className="split-b">
			<Panel name={defaultPanel.name} />
			<Panel name={defaultPanel.name} />
			<Panel name={defaultPanel.name} />
			<Panel name={defaultPanel.name} />
		</main>
	)
};

export default Terminal;
