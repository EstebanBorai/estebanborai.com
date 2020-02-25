import React from 'react';
import { GitHubContext } from 'context';

const getFromGitHub = async (): Promise<GitHubContext> => {
  const URL = 'https://api.github.com/users/estebanborai/repos';
  const ctx: GitHubContext = {
    isAvailable: true,
    languages: null,
    repos: null
  };

  const request = await fetch(URL);
  const repoData = await request.json();

  ctx.languages = {};
  ctx.repos = [];

  repoData.forEach((repo) => {
    const repository: Repo = {};

    repository.name = repo.name;
    repository.language = repo.language;
    repository.description = repo.description;

    if (!(repo.language in ctx.languages)) {
      ctx[repo.language] = 1;
    }

    if (repo.language in ctx.languages) {
      ctx.languages[repo.language] = ctx.languages[repo.language] + 1;
    }
  });

  return ctx;
}

const Profile = (): JSX.Element => {
  const { state, dispatch } = React.useContext(GitHubContext);

  React.useEffect(() => {
    async function fetchGitHub() {
      const data = await getFromGitHub();

      dispatch({ type: 'FETCH', data });
    }

    fetchGitHub();
  }, []);

  return (
    <section className="web-section" id="profile">
      <h2>Profile</h2>
      Im {state.repos && state.repos[0].name}
    </section>
  );
}

export default Profile;
