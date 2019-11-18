import React from 'react';
import PropTypes from 'prop-types';
import TechnologyChip from 'components/TechnologyChip';

const Job = ({
	company, position, dateStarted, dateEnded, about, technologies,
}) => {
	const [isAboutOpen, setAboutOpen] = React.useState(false);

	const dateStartedObj = new Date(dateStarted);
	const dateEndedObj = dateEnded ? new Date(dateEnded) : null;

	const timelapse = {
		start: `${dateStartedObj.getMonth()} ${dateStartedObj.getFullYear()}`,
		end: dateEnded ? `${dateEndedObj.getMonth()} ${dateEndedObj.getFullYear()}` : 'Now',
	};

	const handleClickShowMore = () => {
		if (isAboutOpen) {
			setAboutOpen(false);
		} else {
			setAboutOpen(true);
		}
	};

	return (
		<li className="job-entry">
			<main>
				<div>
					<img src="" alt="" height="64" width="64" className="company-logo" />
					<article>
						<h3 className="company-name">{company}</h3>
						<strong className="job-position">{position}</strong>
						<span className="timelapse">
							{timelapse.start}
							{' '}
-
							{' '}
							{timelapse.end}
						</span>
					</article>
				</div>
				<button type="button" onClick={handleClickShowMore}>
					{isAboutOpen ? 'Hide' : 'Show more'}
				</button>
			</main>
			{
				isAboutOpen
					? (
						<article className="about-job">
							<p>{about}</p>
							<ul className="technologies">
								{
									technologies.map(tech => (
										<TechnologyChip title={tech} />
									))
								}
							</ul>
						</article>
					)
					: null
			}
		</li>
	);
};

Job.propTypes = {
	company: PropTypes.string.isRequired,
	position: PropTypes.string.isRequired,
	dateStarted: PropTypes.instanceOf(Date).isRequired,
	dateEnded: PropTypes.instanceOf(Date),
	about: PropTypes.string.isRequired,
	technologies: PropTypes.arrayOf(PropTypes.string).isRequired,
};

Job.defaultProps = {
	dateEnded: null,
};

export default Job;
