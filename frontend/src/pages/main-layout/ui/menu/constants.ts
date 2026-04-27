import {
  IconMenu2,
  IconMessageCircle2,
  IconRobot,
  IconSettings,
  IconUser,
} from '@tabler/icons-react';

import { CHARACTER_LIST_WINDOW_KEY } from '@entities/character';
import { PERSONA_LIST_WINDOW_KEY } from '@entities/persona';

export const MENU_ITEMS = [
  {
    title: 'menu.personas',
    windowId: PERSONA_LIST_WINDOW_KEY,
    icon: IconUser,
  },
  {
    title: 'menu.characters',
    windowId: CHARACTER_LIST_WINDOW_KEY,
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
