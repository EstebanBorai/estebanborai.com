import React from 'react';
import './summary.scss';

function Summary(): JSX.Element {
  return (
    <section id="summary" className="site-section">
      <h2>Summary</h2>
      <p className="text-block">
        I'm a Software Developer living in The Autonomous City of Buenos Aires.
        Interested in learning new technologies gaining experience in industry of
        software.
      </p>
      <h3>Brief</h3>
      <p className="text-block">
        I have been working in the Software Industry since <b>2017</b> when I started&nbsp;
        a position as a <b>Front-End Developer</b> using ReactJS to write multiple CMS,
        chats, and internal applications.
        <br />
        All of these applications were written in ReactJS using a Web API (usually REST APIs)
        written in C# (.NET Framework).
      </p>
      <h3>Experience</h3>
      <p className="text-block">
        The most of my experience comes from working in the <i>Front End (Client Side)</i>&nbsp;
        with technlogies like ReactJS and JQuery.
      </p>
      <p className="text-block">
        I have also worked with the <i>Back End (Server Side)</i> writting Web APIs with C# (.NET) and
        NodeJS (Express).
      </p>
      <h3>Interests</h3>
      <p className="text-block">
        I'm interested in learning about different technologies. I spend some of my spare time learning
        other languages, working on side projects, doing tutorials or reading articles.&nbsp;
        Even when the most of my work comes from working with the <b>Front-End</b> I'm also interested in the&nbsp;
        <b>Back-End</b>, some of the projects in are either CLIs, REST APIs, GraphQL servers, Chat Applications
        (written with WebSockets).
      </p>
      <p className="text-block">
        The most of my work can be found in my <a className="link" href="https://github.com/estebanborai" target="_blank">GitHub</a>&nbsp;
        where I keep my projects and almost all of my software related activiy. I enjoy building software with&nbsp;
        other people and contributing with open source projects.
      </p>
    </section>
  );
}

export default Summary;
