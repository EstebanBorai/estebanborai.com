import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import Header from '../Header';
import Router from '../../pages/router';

function Layout(): JSX.Element {
  return (
    <BrowserRouter>
      <div id="layout">
        <Header />
        <main className="flex flex-wrap">
          <div className="w-0 sm:w-0 md:w-0 lg:w-0 xl:w-1/6 h-12"></div>
          <div className="w-full sm:w-full md:w-full lg:w-full xl:w-4/6 h-12">
            <Router />
          </div>
          <div className="w-0 sm:w-0 md:w-0 lg:w-0 xl:w-1/6 h-12"></div>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default Layout;
