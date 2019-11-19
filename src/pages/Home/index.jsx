import React from 'react';
import { useTranslation } from 'react-i18next';

const Home = () => {
	const { t } = useTranslation();

	return (
		<section className="page">
			<article>
				<h3>{t('title')}</h3>
				<p>
				Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur
				velit architecto excepturi nostrum veniam inventore consequuntur beatae
				facere, delectus laborum. Beatae labore voluptate aliquid illo voluptatem
				aliquam veritatis accusantium nihil!
				</p>
			</article>
		</section>
	);
};

export default Home;
