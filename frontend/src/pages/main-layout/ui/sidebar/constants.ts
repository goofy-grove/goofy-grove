import {
  IconMessageCircle2,
  IconRobot,
  IconSettings,
  IconUser,
} from '@tabler/icons-react';

export const SIDEBAR_ITEMS = [
  {
    title: 'menu.personas',
    to: '/personas',
    icon: IconUser,
  },
  {
    title: 'menu.characters',
    to: '/characters',
    icon: IconRobot,
  },
  {
    title: 'menu.chats',
    to: '/chats',
    icon: IconMessageCircle2,
  },
  {
    title: 'menu.settings',
    to: '/settings',
    icon: IconSettings,
  },
] as const;
