import { useEffect, useEffectEvent } from 'react';

import { DEFAULT_HOTKEYS } from './defaults';
import { matchKeyCombo } from './match';
import { useHotkeysStore } from './store';

import type { HotkeyId, UseHotkeyOptions } from './types';

export const useHotkey = (
  id: HotkeyId,
  handler?: () => void,
  options: UseHotkeyOptions = {},
) => {
  const { enabled = true } = options;
  const isEnabled = enabled && !!handler;
  const combo = useHotkeysStore(
    (state) => state.overrides[id] ?? DEFAULT_HOTKEYS[id],
  );

  const onHotkey = useEffectEvent(() => {
    handler?.();
  });

  useEffect(() => {
    if (!isEnabled) {
      return;
    }

    const handleKeyDown = (event: KeyboardEvent) => {
      if (!matchKeyCombo(event, combo)) {
        return;
      }

      event.preventDefault();
      onHotkey();
    };

    document.addEventListener('keydown', handleKeyDown);

    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isEnabled, combo]);
};
