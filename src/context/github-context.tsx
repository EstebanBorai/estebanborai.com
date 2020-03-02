import React from 'react';
import ErrorContext from './error-context';

interface LanguageMap {
  [key: string]: number;
}

interface Repo {
  name: string;
  description: string;
  language: string;
}

interface GitHubContext {
  isAvailable: boolean;
  isFetching: boolean;
  isError: unknown;
  repos: Repo[];
  languages: LanguageMap;
}

interface GitHubData {
  repos: Repo[];
  languages: LanguageMap;
}

interface GitHubReposItem {
  name: string;
  htmlUrl: string;
  description: string;
  fork: boolean;
  languagesURL: string;
  cloneURL: string;
  language: string;
  createdAt: Date;
  updatedAt: Date;
}

const GitHubContextContainer = React.createContext<GitHubContext>();

GitHubContextContainer.displayName = 'GitHubContext';

async function gatherStats(): Promise<GitHubData> {
  throw new Error('FATALITY');
  let results: GitHubReposItem[];

  try {
    const data = await fetch('https://api.github.com/users/estebanborai/repos');

    if (data.status > 300) {
      throw new Error(data.status);
    }

    results = await data.json();
  } catch (fetchError) {
    throw fetchError;
  }
    const ctx: GitHubData = {};

    const languages: LanguageMap = {};
    const repos: Repo[] = [];

  try {
    results.forEach((repo: GitHubReposItem) => {
      if (repo.language in languages) {
        languages[repo.language]++;
      } else {
        languages[repo.language] = 1;
      }

      repos.push({
        name: repo.name,
        description: repo.description,
        language: repo.language
      });
    });

    return {
      languages,
      repos
    }
  } catch (err) {
    throw err;
  }
}

export function GitHubContextProvider(props: unknown) {
  const [value, setValue] = React.useState<GitHubContext>({
    isAvailable: false,
    isFetching: false,
    isError: null,
    repos: 'Sarasa',
    languages: null
  });

  const { updateNetwork } = React.useContext(ErrorContext);

  React.useEffect(() => {
    (async function() {
      try {
        const { languages, repos } = await gatherStats();

        setValue({
          ...value,
          languages,
          repos
        })
      } catch (err) {
        updateNetwork(err);
      }
    })();
  }, []);

  return (
    <GitHubContextContainer.Provider value={value}>
      {props.children}
    </GitHubContextContainer.Provider>
  );
}

export const GitHubContextConsumer = GitHubContextContainer.Consumer;

export default GitHubContextContainer;
