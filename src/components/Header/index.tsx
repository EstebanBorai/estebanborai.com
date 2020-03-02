import React from 'react';
import './header.scss';
import LinkList from './LinkList';
import LinkItem from './LinkItem';
import { faGithub, faStackOverflow, faDev, faTwitter, faGitlab } from '@fortawesome/free-brands-svg-icons';

interface HeaderProps {
  isNav: boolean;
}

const Header = ({ isNav }: HeaderProps): JSX.Element => {
  return (
    <header id="header" className={isNav && "top"}>
      <div className="personal">
        <img id="avatar" src="https://avatars2.githubusercontent.com/u/34756077?s=460&v=4" height="150" width="150" />
        <h1>Esteban Borai</h1>
      </div>
      <nav>
        <ul>
          {/* <li>Profile</li> */}
        </ul>
      </nav>
      <LinkList>
        <LinkItem icon={faGithub} href="https://github.com/estebanborai" />
        <LinkItem icon={faGitlab} href="https://gitlab.com/estebanborai" />
        <LinkItem icon={faStackOverflow} href="https://stackoverflow.com/users/9888500/esteban-borai?tab=topactivity" />
        <LinkItem icon={faDev} href="https://dev.to/estebanborai" />
        <LinkItem icon={faTwitter} href="https://twitter.com/estebanborai" />
      </LinkList>
    </header>
  );
};

export default Header;
