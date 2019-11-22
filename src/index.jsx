import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from 'components/App';
import { IntlProvider } from 'react-intl';
import ESTranslations from './locales/es/translations.json';
import ENTranslations from './locales/en/translations.json';

const locales = {
	es: ESTranslations,
	en: ENTranslations,
};

const currentLocale = 'en';

const WrappedApp = () => (
	<IntlProvider locale={currentLocale} defaultLocale="en" messages={locales[currentLocale]}>
		<App />
	</IntlProvider>
);

ReactDOM.render(
	<WrappedApp />,
	document.getElementById('app'),
);
