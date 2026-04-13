import {
  Button,
  LocaleSwitcher,
  registerWindow,
  useCurrentWindow,
  useWindow,
  Window,
  type WindowProps,
} from '@shared/ui';

import type { FC } from 'react';

export const MainPageWindow: FC<WindowProps> = (props) => {
  const { minimizeWindow, isWindowMaximized } = useCurrentWindow();

  return (
    <Window {...props}>
      <h1>main page window</h1>
      {isWindowMaximized && <Button onClick={minimizeWindow}>Minimize</Button>}
    </Window>
  );
};

registerWindow('main-page-window', MainPageWindow);

export const MainPage = () => {
  const { openWindow, isWindowOpen, closeWindow, maximizeWindow } =
    useWindow('main-page-window');
  const { openWindow: openAnotherWindow } = useWindow('main-page-window');

  const handleToggleWindow = () =>
    isWindowOpen ? closeWindow() : openWindow({ title: 'Main Page' });

  return (
    <div>
      <h1>main page</h1>
      <Button onClick={handleToggleWindow}>
        {isWindowOpen ? 'Close' : 'Open'} Main Page Window
      </Button>
      <Button onClick={() => openAnotherWindow({ title: 'Main Page' })}>
        Open Another Window
      </Button>
      {isWindowOpen && <Button onClick={maximizeWindow}>Maximize</Button>}
      <LocaleSwitcher />
    </div>
  );
};
