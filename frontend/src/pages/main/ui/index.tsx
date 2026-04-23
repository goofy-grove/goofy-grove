import {
  IconAlertTriangle,
  IconCheck,
  IconInfoCircle,
  IconSparkles,
  IconX,
} from '@tabler/icons-react';

import {
  Button,
  LocaleSwitcher,
  Text,
  Window,
  registerWindow,
  useCurrentWindow,
  useWindow,
  type ButtonColor,
  type WindowProps,
} from '@shared/ui';

import type { FC } from 'react';

const buttonColors: ButtonColor[] = [
  'default',
  'error',
  'warning',
  'success',
  'info',
];

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
    isWindowOpen
      ? closeWindow()
      : openWindow({ title: 'Main Page', defaultHeight: 'max-content' });

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

      <Text tag="h2">Button variants and colors</Text>
      <Text tag="h3">Default</Text>
      <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
        {buttonColors.map((color) => (
          <Button key={`default-${color}`} color={color}>
            {color}
          </Button>
        ))}
      </div>

      <Text tag="h3">Ghost</Text>
      <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
        {buttonColors.map((color) => (
          <Button key={`ghost-${color}`} variant="ghost" color={color}>
            {color}
          </Button>
        ))}
      </div>

      <Text tag="h3">With icons</Text>
      <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
        <Button leftIcon={<IconSparkles size={16} />}>default</Button>
        <Button color="success" leftIcon={<IconCheck size={16} />}>
          success
        </Button>
        <Button color="warning" leftIcon={<IconAlertTriangle size={16} />}>
          warning
        </Button>
        <Button color="error" leftIcon={<IconX size={16} />}>
          error
        </Button>
        <Button
          variant="ghost"
          color="info"
          rightIcon={<IconInfoCircle size={16} />}
        >
          info
        </Button>
      </div>
    </div>
  );
};
