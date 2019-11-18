import React from 'react';
import Jobs from './Jobs';

const About = () => (
	<section className="page">
		<h2>About</h2>
		<article>
			<h3>Software Developer</h3>
			<p>
				<em>
          Esteban is a software development enthusiast, he is open to
          learn and work with different technologies, work in multiple
          kind of projects either web based software like websites and
          web applications, mobile applications or desktop applications,
          even when the most of his experience comes from working
          with ReactJS writting Web Applications
				</em>
        The majority of my experience comes greatly from ReactJS. I am
        also interested in technologies like Golang and Python.

        I spend some of my spare time writting applications, where I
        experiment with technologies I'm interested in, even when
        I'm not currently woking with tat technology at the moment.

        This helps me to open my mind to other languages, learn about other
        programming languages and come up with some ideas to share at work
        with colleages.
			</p>
		</article>
		<Jobs />
	</section>
);

export default About;
