import React from 'react';
import Header from 'components/Header';
import ErrorMessage from 'components/ErrorMessage';
import Profile from 'components/Profile';
import { GitHubContextProvider, ErrorContextProvider } from '../context';
import CautionDeveloperWorkingOverhead from './CautionDeveloperWorkingOverhead';

const App = (): JSX.Element => (
  <ErrorContextProvider>
    <GitHubContextProvider>
      <ErrorMessage />
      <>
        <Header isNav={true} />
        <main>
          <Profile />
        </main>
      </>
    </GitHubContextProvider>
  </ErrorContextProvider>
);

export default App;
