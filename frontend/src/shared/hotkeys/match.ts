import type { KeyCombo } from './types';

const normalizeKey = (key: string) =>
  key.length === 1 ? key.toLowerCase() : key;

export const matchKeyCombo = (event: KeyboardEvent, combo: KeyCombo) => {
  if (normalizeKey(event.key) !== normalizeKey(combo.key)) {
    return false;
  }

  if (Boolean(combo.ctrl) !== event.ctrlKey) {
    return false;
  }

  if (Boolean(combo.shift) !== event.shiftKey) {
    return false;
  }

  if (Boolean(combo.alt) !== event.altKey) {
    return false;
  }

  if (Boolean(combo.meta) !== event.metaKey) {
    return false;
  }

  return true;
};
