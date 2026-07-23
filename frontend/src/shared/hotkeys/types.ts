export type HotkeyId = 'modal.close';

export type KeyCombo = {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
};

export type HotkeyDefinition = {
  id: HotkeyId;
  combo: KeyCombo;
};

export type HotkeysStore = {
  overrides: Partial<Record<HotkeyId, KeyCombo>>;
  setOverride: (id: HotkeyId, combo: KeyCombo) => void;
  resetOverride: (id: HotkeyId) => void;
};

export type UseHotkeyOptions = {
  enabled?: boolean;
};
