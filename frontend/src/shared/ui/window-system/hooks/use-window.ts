import { windowService } from '../services';

import { useWindowSystem } from './use-window-system';

export const useWindow = <T>(windowId: string) => {
  const { activeWindows } = useWindowSystem();

  const windowState = activeWindows.find((window) => window.id === windowId);

  const openWindow = (props?: T) => {
    windowService.openWindow(windowId, props);
  };

  const closeWindow = () => {
    windowService.closeWindow(windowId);
  };

  const maximizeWindow = () => {
    windowService.maximizeWindow(windowId);
  };

  const minimizeWindow = () => {
    windowService.minimizeWindow(windowId);
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
