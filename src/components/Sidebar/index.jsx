import React from 'react';
import './sidebar.scss';

const Sidebar = () => (
	<aside id="profile-sidebar">
		<section>
			<img
				src="https://avatars2.githubusercontent.com/u/34756077?s=460&v=4"
				alt="Esteban's Profile Picture"
				height="271"
				width="271"
			/>
			<article>
				<h1>Esteban Borai</h1>
			</article>
		</section>
		<ol>
			<li>Buenos Aires, Argentina</li>
			<li>estebanborai@gmail.com</li>
		</ol>
	</aside>
);

export default Sidebar;
