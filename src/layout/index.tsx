import React from 'react';
import Header from './Header';
import Summary from './Summary';

const Layout = (): JSX.Element => (
  <div>
    <Header />
    <main>
      <Summary />
    </main>
  </div>
);

export default Layout;
