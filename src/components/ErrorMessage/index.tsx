import React from 'react';
import './error-message.scss';
import { ErrorContext } from 'context';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faWifi, faBomb } from '@fortawesome/free-solid-svg-icons';

const ErrorMessage = (): JSX.Element => {
  const errorContext = React.useContext(ErrorContext);

  return (
    <div id="error-message" className="fatal">
      <span className="icon-container">
        <FontAwesomeIcon icon={faBomb} />
      </span>
      <span>Something is wrong!</span>
    </div>
  )
}

export default ErrorMessage;
