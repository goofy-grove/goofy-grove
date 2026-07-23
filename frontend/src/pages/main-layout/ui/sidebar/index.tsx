import { Link, useRouterState } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { Button, useBreakpoints } from '@shared/ui';

import { SIDEBAR_ITEMS } from './constants';

import type { FC } from 'react';

import './styles.scss';

export const Sidebar: FC = () => {
  const { t } = useTranslation();
  const { isTabletSm } = useBreakpoints();
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  });

  return (
    <nav className={`sidebar ${isTabletSm ? 'sidebar--collapsed' : ''}`}>
      <div className="sidebar__items">
        {SIDEBAR_ITEMS.map(({ title, to, icon: Icon }) => {
          const isActive = pathname === to || pathname.startsWith(`${to}/`);

          return (
            <Link key={to} to={to} className="sidebar__link">
              <Button
                className={`sidebar__button ${isActive ? 'sidebar__button--active' : ''}`}
                variant="ghost"
                leftIcon={<Icon size={isTabletSm ? 28 : 24} />}
              >
                {isTabletSm ? '' : t(title)}
              </Button>
            </Link>
          );
        })}
      </div>
    </nav>
  );
};
