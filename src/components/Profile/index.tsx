import React from 'react';
import './profile.scss';
import { GitHubContext } from 'context';
import LanguagesBars from './LanguagesBars';

const Profile = (): JSX.Element => {
  const githubContext = React.useContext(GitHubContext);

  return (
    <section className="web-section" id="profile">
      <h2>Profile</h2>
      <div className="profile-contents">
        <LanguagesBars />
        <article>
          <h3>These are the languages I have tried at least once..</h3>
          <p>
            In my spare time I work on side projects where I take a chance to give a try to different architectures,
            languages, frameworks, tools or components in order to learn more about the ecosystem.
          </p>
        </article>
      </div>
    </section>
  );
}

export default Profile;
