import { type FC, type PropsWithChildren } from 'react';

import { WindowContext } from '../services';
import { useWindowRegistryStore, useWindowStore } from '../store';

export const WindowsProvider: FC<PropsWithChildren> = ({ children }) => {
  const windows = useWindowStore((state) => state.windows);
  const getWindow = useWindowRegistryStore((state) => state.get);

  return (
    <>
      {children}

      {windows.map((window) => {
        const WindowComponent = getWindow(window.type)?.component;

        if (!WindowComponent) {
          return null;
        }

        return (
          <WindowContext.Provider value={window} key={window.instanceId}>
            <WindowComponent {...window.props} />
          </WindowContext.Provider>
        );
      })}
    </>
  );
};
