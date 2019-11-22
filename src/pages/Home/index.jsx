import React from 'react';
import { useIntl } from 'react-intl';

const Home = () => {
	const { messages } = useIntl();

	return (
		<section className="page">
			<article>
				<h3>{messages.home.title}</h3>
				<p>{messages.home.entry}</p>
			</article>
		</section>
	);
};

export default Home;
