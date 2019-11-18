import React from 'react';
import PropTypes from 'prop-types';

const Job = ({
	company, position, dateStarted, dateEnded,
}) => {
	const dateStartedObj = new Date(dateStarted);
	const dateEndedObj = dateEnded ? new Date(dateEnded) : null;

	const timelapse = {
		start: `${dateStartedObj.getMonth()} ${dateStartedObj.getFullYear()}`,
		end: dateEnded ? `${dateEndedObj.getMonth()} ${dateEndedObj.getFullYear()}` : 'Now',
	};

	return (
		<li className="job-entry">
			<img src="" alt="" height="64" width="64" />
			<div>
				<h3 className="company-name">{company}</h3>
				<strong className="job-position">{position}</strong>
				<span className="timelapse">
					{timelapse.start}
					{' '}
-
					{' '}
					{timelapse.end}
				</span>
			</div>
		</li>
	);
};

Job.propTypes = {
	company: PropTypes.string.isRequired,
	position: PropTypes.string.isRequired,
	dateStarted: PropTypes.instanceOf(Date).isRequired,
	dateEnded: PropTypes.instanceOf(Date),
};

Job.defaultProps = {
	dateEnded: null,
};

export default Job;
