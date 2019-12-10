import React from 'react';
import './sidebar.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faEnvelope, faMapMarkerAlt } from '@fortawesome/free-solid-svg-icons';
import { faGithub } from '@fortawesome/free-brands-svg-icons';

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
			<li><FontAwesomeIcon icon={faMapMarkerAlt} />&nbsp;Buenos Aires, Argentina</li>
			<li><FontAwesomeIcon icon={faEnvelope} />&nbsp;estebanborai@gmail.com</li>
			<li><FontAwesomeIcon icon={faGithub} />&nbsp;estebanborai</li>
		</ol>
	</aside>
);

export default Sidebar;
