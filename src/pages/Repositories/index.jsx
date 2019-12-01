import React from 'react';
import { useIntl } from 'react-intl';

const Repositories = () => {
	const { messages } = useIntl();
	return (
		<ol>
			<li>{messages.nav.repositories}</li>
		</ol>
	);
};

export default Repositories;
