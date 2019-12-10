import React from 'react';
import './sidebar.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faEnvelope, faMapMarkerAlt } from '@fortawesome/free-solid-svg-icons';
import { faGithub } from '@fortawesome/free-brands-svg-icons';

const ReferenceLink = () => (
	<li>
		<FontAwesomeIcon icon={faMapMarkerAlt} />&nbsp;Buenos Aires, Argentina</li>
);

export default ReferenceLink;
