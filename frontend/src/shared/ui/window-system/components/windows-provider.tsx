import { type FC, type PropsWithChildren } from 'react';

import { WindowContext, windowRegistry } from '../services';
import { useWindowStore } from '../store';

export const WindowsProvider: FC<PropsWithChildren> = ({ children }) => {
  const windows = useWindowStore((state) => state.windows);
  const closeWindow = useWindowStore((state) => state.closeWindow);
  const maximizeWindow = useWindowStore((state) => state.maximizeWindow);
  const minimizeWindow = useWindowStore((state) => state.minimizeWindow);

  return (
    <>
      {children}

      {windows.map((window) => {
        const WindowComponent = windowRegistry.get(window.type);

        if (!WindowComponent) {
          return null;
        }

        return (
          <WindowContext.Provider value={window} key={window.instanceId}>
            <WindowComponent
              {...window.props}
              id={window.instanceId}
              isMaximized={window.isMaximized}
              onClose={() => closeWindow(window.instanceId)}
              onMaximize={() => maximizeWindow(window.instanceId)}
              onMinimize={() => minimizeWindow(window.instanceId)}
            />
          </WindowContext.Provider>
        );
      })}
    </>
  );
};
