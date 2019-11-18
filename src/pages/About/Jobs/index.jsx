import React from 'react';
import './jobs.scss';
import Job from './Job';

const Jobs = () => {
	const [jobs, setJobs] = React.useState(null);

	React.useEffect(() => {
		fetch('https://raw.githubusercontent.com/estebanborai/estebanborai.github.io/master/data/background.json')
			.then(res => res.json())
			.then((data) => {
				setJobs(data);
			});
	}, []);

	return (
		<article id="job-entries">
			<h3>Background</h3>
			<ol>
				{
					jobs ? jobs.map(job => (
						<Job
							about={job.about}
							company={job.company}
							position={job.position}
							dateStarted={job.dateStarted}
							dateEnded={job.dateEnded}
							technologies={job.technologies}
						/>
					)) : <span>Please Wait</span>
				}
			</ol>
		</article>
	);
};

export default Jobs;
