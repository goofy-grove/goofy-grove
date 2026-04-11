import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
// eslint-disable-next-line import/no-unresolved
import { v4 as uuidv4 } from 'uuid';

import type { WindowState } from './types';

export const useWindowStore = create<WindowState>()(
  devtools(
    (set) => ({
      windows: [],

      openWindow: (type: string, props?: Record<string, unknown>) => {
        const instanceId = `${type}-${uuidv4()}`;

        set(
          (state) => ({
            windows: [
              ...state.windows,
              {
                instanceId,
                lastInteraction: Date.now(),
                type,
                isMaximized: false,
                props,
              },
            ],
          }),
          false,
          { type: 'window-store/openWindow', payload: { type, props } },
        );

        return instanceId;
      },

      closeWindow: (instanceId: string) =>
        set(
          (state) => ({
            windows: state.windows.filter((w) => w.instanceId !== instanceId),
          }),
          false,
          { type: 'window-store/closeWindow', payload: { instanceId } },
        ),

      maximizeWindow: (instanceId: string) =>
        set(
          (state) => ({
            windows: state.windows.map((w) =>
              w.instanceId === instanceId ? { ...w, isMaximized: true } : w,
            ),
          }),
          false,
          { type: 'window-store/maximizeWindow', payload: { instanceId } },
        ),

      minimizeWindow: (instanceId: string) =>
        set(
          (state) => ({
            windows: state.windows.map((w) =>
              w.instanceId === instanceId ? { ...w, isMaximized: false } : w,
            ),
          }),
          false,
          { type: 'window-store/minimizeWindow', payload: { instanceId } },
        ),
    }),
    { name: 'window-store' },
  ),
);
