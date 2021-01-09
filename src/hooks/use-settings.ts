import { useContext } from 'react';

import SettingsContext from '@estebanborai.com/context/settings';

import type { SettingsContextValue } from '@estebanborai.com/context/settings';

export default function useSettings(): SettingsContextValue {
  const settingsContext = useContext(SettingsContext);

  return settingsContext;
}
