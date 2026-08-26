import { create } from 'zustand';

import type { HotkeyId, HotkeysStore, KeyCombo } from './types';

export const useHotkeysStore = create<HotkeysStore>((set) => ({
  overrides: {},

  setOverride: (id: HotkeyId, combo: KeyCombo) =>
    set((state) => ({
      overrides: {
        ...state.overrides,
        [id]: combo,
      },
    })),

  resetOverride: (id: HotkeyId) =>
    set((state) => {
      const rest = { ...state.overrides };

      delete rest[id];

      return { overrides: rest };
    }),
}));
