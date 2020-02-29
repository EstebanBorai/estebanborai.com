import React from 'react';
import Header from 'components/Header';
import Profile from 'components/Profile';
import { GitHubContextProvider } from '../context';
import CautionDeveloperWorkingOverhead from './CautionDeveloperWorkingOverhead';

const App = (): JSX.Element => (
  <GitHubContextProvider>
    <>
      <Header isNav={true} />
      <main>
        {/* <Profile /> */}
        <CautionDeveloperWorkingOverhead />
      </main>
    </>
  </GitHubContextProvider>
);

export default App;
