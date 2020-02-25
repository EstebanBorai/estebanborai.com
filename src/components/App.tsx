import React from 'react';
import Heading from 'components/heading';
import Profile from 'components/Profile';
import { GitHubContextProvider } from '../context';

const App = (): JSX.Element => (
  <GitHubContextProvider>
    <>
      <Heading isNav={true} />
      <main>
        <Profile />
      </main>
    </>
  </GitHubContextProvider>
);

export default App;
