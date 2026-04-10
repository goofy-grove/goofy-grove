import { create } from 'zustand';
// eslint-disable-next-line import/no-unresolved
import { v4 as uuidv4 } from 'uuid';

import type { WinowState } from './types';

export const useWindowStore = create<WinowState>((set) => ({
  windows: [],

  openWindow: (type: string, props?: Record<string, unknown>) => {
    const instanceId = `${type}-${uuidv4()}`;

    set((state) => ({
      windows: [
        ...state.windows,
        {
          instanceId,
          lastInteraction: Date.now(),
          type,
          isMaximized: false,
          isOpen: true,
          props,
        },
      ],
    }));

    return instanceId;
  },

  closeWindow: (instanceId: string) =>
    set((state) => ({
      windows: state.windows.filter((w) => w.instanceId !== instanceId),
    })),

  maximizeWindow: (instanceId: string) =>
    set((state) => ({
      windows: state.windows.map((w) =>
        w.instanceId === instanceId ? { ...w, isMaximized: true } : w,
      ),
    })),

  minimizeWindow: (instanceId: string) =>
    set((state) => ({
      windows: state.windows.map((w) =>
        w.instanceId === instanceId ? { ...w, isMaximized: false } : w,
      ),
    })),
}));
