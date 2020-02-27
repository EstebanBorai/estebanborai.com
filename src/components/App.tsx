import React from 'react';
import Heading from 'components/heading';
import Profile from 'components/Profile';
import { GitHubContextProvider } from '../context';
import CautionDeveloperWorkingOverhead from './CautionDeveloperWorkingOverhead';

const App = (): JSX.Element => (
  <GitHubContextProvider>
    <>
      <Heading isNav={true} />
      <main>
        {/* <Profile /> */}
        <CautionDeveloperWorkingOverhead />
      </main>
    </>
  </GitHubContextProvider>
);

export default App;
