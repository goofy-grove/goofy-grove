import { useTranslation } from 'react-i18next';

import { Button } from '../../../../shared/ui';

import { MENU_ITEMS } from './constants';

import type { FC } from 'react';
import type { MenuProps } from './types';

import './styles.scss';

export const Menu: FC<MenuProps> = ({ isCollapsed }) => {
  const { t } = useTranslation();

  return MENU_ITEMS.map(({ title, windowId, icon: Icon }) => (
    <Button
      key={windowId}
      className={isCollapsed ? 'full-rounded' : ''}
      variant="ghost"
      leftIcon={<Icon size={isCollapsed ? 28 : 24} />}
    >
      {isCollapsed ? '' : t(title)}
    </Button>
  ));
};
