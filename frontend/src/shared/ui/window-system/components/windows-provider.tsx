import { type FC, type PropsWithChildren } from 'react';

import { WindowContext } from '../services';
import { useWindowRegistryStore, useWindowStore } from '../store';

import { WINDOW_BASE_Z_INDEX } from './constants';

export const WindowsProvider: FC<PropsWithChildren> = ({ children }) => {
  const windows = useWindowStore((state) => state.windows);
  const sortedWindows = windows.toSorted(
    (a, b) => a.lastInteraction - b.lastInteraction,
  );
  const getWindow = useWindowRegistryStore((state) => state.get);

  return (
    <>
      {children}

      {windows.map((window) => {
        const WindowComponent = getWindow(window.type)?.component;

        if (!WindowComponent) {
          return null;
        }

        const index = sortedWindows.findIndex(
          (w) => w.instanceId === window.instanceId,
        );

        return (
          <WindowContext.Provider
            value={{ ...window, zIndex: index + WINDOW_BASE_Z_INDEX }}
            key={window.instanceId}
          >
            <WindowComponent {...window.props} />
          </WindowContext.Provider>
        );
      })}
    </>
  );
};
