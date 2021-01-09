import React from 'react';
import { ThemeProvider } from 'styled-components';

import ErrorBoundary from '@estebanborai.com/components/ErrorBoundary';
import Layout from '@estebanborai.com/components/Layout';
import SettingsContext, {
  SettingsContextProvider,
} from '@estebanborai.com/context/settings';
import theme from '@estebanborai.com/styles/theme';

import '@estebanborai.com/styles/global.css';

export default function App(): JSX.Element {
  return (
    <React.StrictMode>
      <ErrorBoundary>
        <SettingsContextProvider>
          <SettingsContext.Consumer>
            {(value) => (
              <ThemeProvider theme={theme[value.theme]}>
                <Layout />
              </ThemeProvider>
            )}
          </SettingsContext.Consumer>
        </SettingsContextProvider>
      </ErrorBoundary>
    </React.StrictMode>
  );
}

// Hot Module Replacement (HMR) - Remove this snippet to remove HMR.
// Learn more: https://www.snowpack.dev/#hot-module-replacement
if (import.meta.hot) {
  import.meta.hot.accept();
}
