import React from 'react';
import { GitHubContext } from 'context';

const Profile = (): JSX.Element => {
  const githubContext = React.useContext(GitHubContext);

  return (
    <section className="web-section" id="profile">
      <h2>Profile</h2>
      {JSON.stringify(githubContext.isError)}
      {JSON.stringify(githubContext.repos)}
      {JSON.stringify(githubContext.languages)}
    </section>
  );
}

export default Profile;
