import React from 'react';
import './jobs.scss';
import Job from './Job';

const Jobs = () => {
	const [jobs, setJobs] = React.useState(null);

	React.useEffect(() => {
		fetch('http://0.0.0.0:8000/background.json').then(res => res.json()).then((data) => {
			setJobs(data);
		});
	});

	return (
		<article id="job-entries">
			<h3>Background</h3>
			<ol>
				{
					jobs ? jobs.map(job => (
						<Job
							company={job.company}
							position={job.position}
							dateStarted={job.dateStarted}
							dateEnded={job.dateEnded}
						/>
					)) : null
				}
			</ol>
		</article>
	);
};

export default Jobs;
