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
    modalId: 'personas-list',
    icon: IconUser,
  },
  {
    title: 'menu.characters',
    modalId: 'characters-list',
    icon: IconRobot,
  },
  {
    title: 'menu.chats',
    modalId: 'chats-list',
    icon: IconMessageCircle2,
  },
  {
    title: 'menu.settings',
    modalId: 'settings',
    icon: IconSettings,
  },
  {
    title: 'menu.other',
    modalId: 'other',
    icon: IconMenu2,
  },
];
