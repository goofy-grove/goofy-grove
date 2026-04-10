import {
  Button,
  useWindow,
  Window,
  windowRegistry,
  type WindowProps,
} from '../../../shared/ui';

import type { FC } from 'react';

export const MainPageWindow: FC<
  Pick<WindowProps, 'onClose' | 'onMaximize'>
> = ({ onClose, onMaximize, ...props }) => (
  <Window
    id="main-page-window"
    {...props}
    onClose={onClose}
    onMaximize={onMaximize}
  >
    <h1>main page window</h1>
  </Window>
);

windowRegistry.register('main-page-window', MainPageWindow);

export const MainPage = () => {
  const { openWindow, isWindowOpen, closeWindow } =
    useWindow('main-page-window');

  const handleToggleWindow = () =>
    isWindowOpen ? closeWindow() : openWindow({ title: 'Main Page' });

  return (
    <div>
      <h1>main page</h1>
      <Button onClick={handleToggleWindow}>
        {isWindowOpen ? 'Close' : 'Open'} Main Page Window
      </Button>
    </div>
  );
};
