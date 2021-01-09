import React, { lazy, Suspense } from 'react';
import styled from 'styled-components';
import { BrowserRouter, Route } from 'react-router-dom';

import SuspenseFallback from '@estebanborai.com/components/SuspenseFallback';
import HomePage from '@estebanborai.com/pages/Home';
import Header from './Header';

const LayoutContainer = styled.div`
  background-color: ${(props) => props.theme.layoutBackground};
  color: ${(props) => props.theme.textColor};
  transition: all 250ms ease-out;
`;

const Footer = styled.footer`
  align-items: center;
  box-sizing: border-box;
  display: flex;
  height: 30px;
  justify-content: center;
  padding: 1rem;
`;

const BookshelfPage = lazy(() => import('@estebanborai.com/pages/Bookshelf'));

export default function Layout(): JSX.Element {
  return (
    <LayoutContainer>
      <BrowserRouter>
        <React.Fragment>
          <Header />
          <main>
            <Suspense fallback={<SuspenseFallback />}>
              <Route exact path="/" component={HomePage} />
              <Route path="/bookshelf" component={BookshelfPage} />
            </Suspense>
          </main>
          <Footer>
            <small>Made with code and 🧉 by Esteban Borai</small>
          </Footer>
        </React.Fragment>
      </BrowserRouter>
    </LayoutContainer>
  );
}
