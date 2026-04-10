import { windowService } from '../model';

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

  const isWindowOpen = !!windowState;

  return {
    openWindow,
    closeWindow,
    isWindowOpen,
  };
};
