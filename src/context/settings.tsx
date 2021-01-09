import React, { createContext, useState } from 'react';

import LocalStorage, {
  LocalStorageKey,
} from '@estebanborai.com/utils/local-storage';

export type Theme = 'light' | 'dark';

export type Settings = {
  theme: Theme;
};

export type SettingsContextValue = {
  theme: Theme;
  toggleTheme(): void;
};

const SettingsContext = createContext<SettingsContextValue>({
  theme: LocalStorage.getValue(LocalStorageKey.Theme) || 'dark',
  toggleTheme() {
    return;
  },
});

SettingsContext.displayName = 'SettingsContext';

export function SettingsContextProvider(props: {
  children: JSX.Element | JSX.Element[];
}): JSX.Element {
  const [theme, setTheme] = useState<Theme>(
    LocalStorage.getValue(LocalStorageKey.Theme) || 'dark',
  );

  const toggleTheme = (): void => {
    const next = theme === 'dark' ? 'light' : 'dark';

    LocalStorage.setValue(LocalStorageKey.Theme, next);
    setTheme(next);
  };

  const value = {
    theme,
    toggleTheme,
  };

  return (
    <SettingsContext.Provider value={value}>
      {props.children}
    </SettingsContext.Provider>
  );
}

export default SettingsContext;
