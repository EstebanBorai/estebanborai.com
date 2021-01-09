const LOCAL_STORAGE_KEY_PREFIX = 'estebanborai::dotcom';

export enum LocalStorageKey {
  Theme = 'theme',
}

function setValue<T>(key: LocalStorageKey, value: T): void {
  localStorage.setItem(
    `${LOCAL_STORAGE_KEY_PREFIX}::${key}`,
    JSON.stringify(value),
  );
}

function getValue<T>(key: LocalStorageKey): T | null {
  const keyString = `${LOCAL_STORAGE_KEY_PREFIX}::${key}`;
  const value = localStorage.getItem(keyString);

  if (!value) {
    return null;
  }

  return JSON.parse(value);
}

export default {
  setValue,
  getValue,
};
