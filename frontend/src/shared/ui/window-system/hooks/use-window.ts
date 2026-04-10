import { useState } from 'react';

import { useWindowStore } from '../store';

export const useWindow = <T extends Record<string, unknown>>(
  type: string,
  instanceId?: string,
) => {
  const [currentWindowId, setCurrentWindowId] = useState<string | undefined>(
    instanceId,
  );

  const openWindowByType = useWindowStore((state) => state.openWindow);
  const closeWindowById = useWindowStore((state) => state.closeWindow);
  const maximizeWindowById = useWindowStore((state) => state.maximizeWindow);
  const minimizeWindowById = useWindowStore((state) => state.minimizeWindow);

  const windowState = useWindowStore((state) =>
    state.windows.find((w) => w.instanceId === currentWindowId),
  );

  if (!windowState && currentWindowId) {
    setCurrentWindowId(undefined);
  }

  const openWindow = (props?: T) => {
    if (!currentWindowId) {
      setCurrentWindowId(openWindowByType(type, props));
    }
  };

  const closeWindow = () => {
    if (currentWindowId) {
      closeWindowById(currentWindowId);
      setCurrentWindowId(undefined);
    }
  };

  const maximizeWindow = () => {
    if (currentWindowId) {
      maximizeWindowById(currentWindowId);
    }
  };

  const minimizeWindow = () => {
    if (currentWindowId) {
      minimizeWindowById(currentWindowId);
    }
  };

  const isWindowOpen = !!windowState;

  const isWindowMaximized = !!windowState && windowState?.isMaximized;

  return {
    openWindow,
    closeWindow,
    maximizeWindow,
    minimizeWindow,
    isWindowOpen,
    isWindowMaximized,
  };
};
