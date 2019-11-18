import React from 'react';
import PropTypes from 'prop-types';
import './technology-chip.scss';

const TechnologyChip = ({ title }) => (
	<li className="technology-chip">
		{title}
	</li>
);

TechnologyChip.propTypes = {
	title: PropTypes.string.isRequired,
};

export default TechnologyChip;
