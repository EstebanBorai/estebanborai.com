import React from 'react';
import { ErrorContext } from 'context';

const ErrorMessage = (): JSX.Element => {
  const errorContext = React.useContext(ErrorContext);

  return (
  <h1>{JSON.stringify(errorContext)}</h1>
  )
}

export default ErrorMessage;
