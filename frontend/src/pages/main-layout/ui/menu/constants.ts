import {
  IconMenu2,
  IconMessageCircle2,
  IconRobot,
  IconSettings,
  IconUser,
} from '@tabler/icons-react';

export const MENU_ITEMS = [
  {
    title: 'menu.personas',
    windowId: 'personas-list',
    icon: IconUser,
  },
  {
    title: 'menu.characters',
    windowId: 'characters-list',
    icon: IconRobot,
  },
  {
    title: 'menu.chats',
    windowId: 'chats-list',
    icon: IconMessageCircle2,
  },
  {
    title: 'menu.settings',
    windowId: 'settings',
    icon: IconSettings,
  },
  {
    title: 'menu.other',
    windowId: 'other',
    icon: IconMenu2,
  },
];
