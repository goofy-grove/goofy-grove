import { useTranslation } from 'react-i18next';

import { PERSONA_LIST_WINDOW_KEY } from '@entities/persona';

import { Button, useWindow } from '@shared/ui';

import { MENU_ITEMS } from './constants';

import type { MenuProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const Menu: FC<MenuProps> = ({ isCollapsed }) => {
  const { t } = useTranslation();

  const { openWindow } = useWindow(PERSONA_LIST_WINDOW_KEY);

  const handleOpenWindow = (windowId: string) => {
    if (windowId === PERSONA_LIST_WINDOW_KEY) {
      openWindow({
        title: t('person.window.list_title'),
      });
    }
  };

  return MENU_ITEMS.map(({ title, windowId, icon: Icon }) => (
    <Button
      key={windowId}
      className={isCollapsed ? 'full-rounded' : ''}
      variant="ghost"
      leftIcon={<Icon size={isCollapsed ? 28 : 24} />}
      onClick={() => handleOpenWindow(windowId)}
    >
      {isCollapsed ? '' : t(title)}
    </Button>
  ));
};
