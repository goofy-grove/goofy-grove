import { DEFAULT_HOTKEYS } from './defaults';
import { useHotkeysStore } from './store';

import type { HotkeyId, KeyCombo } from './types';

export const resolveHotkey = (id: HotkeyId): KeyCombo => {
  const override = useHotkeysStore.getState().overrides[id];

  return override ?? DEFAULT_HOTKEYS[id];
};
