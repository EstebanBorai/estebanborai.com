import React from 'react';

const ErrorContextContainer = React.createContext();

ErrorContextContainer.displayName = 'ErrorContext';

export function ErrorContextProvider(props: unknown) {
  const [errorState, setErrorState] = React.useState({
    fatal: null
  });

  const updateFatal = (err) => {
    setErrorState({
      ...errorState,
      fatal: err,
    });
  }

  const updateNetwork = (err) => {
    setErrorState({
      ...errorState,
      network: err.message
    });
  }

  return (
    <ErrorContextContainer.Provider value={{ errorState, updateFatal, updateNetwork }}>
      {props.children}
    </ErrorContextContainer.Provider>
  );
}

export const ErrorContextConsumer = ErrorContextContainer.Consumer;

export default ErrorContextContainer;
