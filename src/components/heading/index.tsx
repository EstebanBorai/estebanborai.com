import * as React from 'react';
import './heading.scss';
import LinkList from './LinkList';
import LinkItem from './LinkItem';
import { faGithub, faStackOverflow, faDev, faTwitter } from '@fortawesome/free-brands-svg-icons';

const Heading = (): JSX.Element => {
  return (
    <section id="heading" className="temp">
      <img id="avatar" src="https://avatars2.githubusercontent.com/u/34756077?s=460&v=4" height="150" width="150" />
      <h1>Esteban Borai</h1>
      <LinkList>
        <LinkItem icon={faGithub} href="https://github.com/estebanborai" />
        <LinkItem icon={faStackOverflow} href="https://stackoverflow.com/users/9888500/esteban-borai?tab=topactivity" />
        <LinkItem icon={faDev} href="https://dev.to/estebanborai" />
        <LinkItem icon={faTwitter} href="https://twitter.com/estebanborai" />
      </LinkList>
    </section>
  );
};

export default Heading;
