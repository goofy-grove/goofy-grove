import { useEffect, useState, type FC, type PropsWithChildren } from 'react';

import {
  WindowContext,
  windowRegistry,
  windowService,
  type WindowState,
} from '../model';

export const WindowProvider: FC<PropsWithChildren> = ({ children }) => {
  const [activeWindows, setActiveWindows] = useState<WindowState[]>([]);

  useEffect(() => {
    const unsubscribeOpen = windowService.onWindowOpen((id, props) =>
      setActiveWindows((prev) => {
        const existingWindow = prev.find((window) => window.id === id);

        return [
          ...prev.filter((window) => window.id !== id),
          existingWindow
            ? { ...existingWindow, lastInteraction: Date.now() }
            : {
                id,
                isMaximized: false,
                lastInteraction: Date.now(),
                props,
              },
        ];
      }),
    );

    const unsubscribeClose = windowService.onWindowClose((id) =>
      setActiveWindows((prev) => prev.filter((window) => window.id !== id)),
    );

    return () => {
      unsubscribeOpen();
      unsubscribeClose();
    };
  }, []);

  const maximizeWindow = (id: string) => {
    setActiveWindows((prev) => {
      const windowToMaximize = prev.find((window) => window.id === id);

      if (!windowToMaximize) {
        return prev;
      }

      return [
        ...prev.filter((window) => window.id !== id),
        {
          ...windowToMaximize,
          isMaximized: !windowToMaximize.isMaximized,
          lastInteraction: Date.now(),
        },
      ];
    });
  };

  return (
    <WindowContext.Provider value={{ activeWindows }}>
      {children}

      {Object.values(activeWindows).map((windowState) => {
        const WindowComponent = windowRegistry.get(windowState.id);

        if (!WindowComponent) {
          return null;
        }

        return (
          <WindowComponent
            key={windowState.id}
            {...windowState.props}
            onClose={() => windowService.closeWindow(windowState.id)}
            onMaximize={() => maximizeWindow(windowState.id)}
          />
        );
      })}
    </WindowContext.Provider>
  );
};
