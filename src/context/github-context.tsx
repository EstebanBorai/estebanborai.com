import React from 'react';

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
  repos: Repo[];
  languages: LanguageMap;
}

const GitHubContextContainer = React.createContext<GitHubContext>();

const initialState: GitHubContext = {
  isAvailable: false,
  repos: null,
  languages: null
};

const reducer = (state, action) => {
  switch (action.type) {
    case 'FETCH':
      console.log(action.data);
      return state;
  }
}

GitHubContextContainer.displayName = 'GitHubContext';

export function GitHubContextProvider(props: unknown) {
  const [state, dispatch] = React.useReducer(reducer, initialState);
  const value = { state, dispatch };

  return (
    <GitHubContextContainer.Provider value={value}>
      {props.children}
    </GitHubContextContainer.Provider>
  );
}

export const GitHubContextConsumer = GitHubContextContainer.Consumer;

export default GitHubContextContainer;
