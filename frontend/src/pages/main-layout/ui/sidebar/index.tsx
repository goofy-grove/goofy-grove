import { IconTrees } from '@tabler/icons-react';
import { Link, useRouterState } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { useBreakpoints } from '@shared/ui';

import { SIDEBAR_ITEMS } from './constants';

import './styles.scss';

export const Sidebar = () => {
  const { t } = useTranslation();
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  });
  const { isMobileSm } = useBreakpoints();

  return (
    <nav className="sidebar" aria-label={t('grove.navigation')}>
      <Link to="/characters" className="sidebar__brand">
        <span className="sidebar__mark">
          <IconTrees size={30} stroke={1.7} />
        </span>

        <span>
          goofy<span className="sidebar__brand-second">grove.</span>
        </span>
      </Link>

      <span className="sidebar__caption">{t('grove.workspace')}</span>

      <div className="sidebar__items">
        {SIDEBAR_ITEMS.map(({ title, to, icon: Icon }) => {
          const isActive = pathname === to || pathname.startsWith(`${to}/`);

          return (
            <Link
              key={to}
              to={to}
              aria-label={t(title)}
              title={t(title)}
              aria-current={isActive ? 'page' : undefined}
              className={`sidebar__link ${isActive ? 'sidebar__link--active' : ''}`}
            >
              <Icon size={isMobileSm ? 32 : 24} stroke={1.7} />

              <span>{t(title)}</span>
            </Link>
          );
        })}
      </div>

      <div className="sidebar__footer">
        <IconTrees size={24} />

        <span>{t('grove.tagline')}</span>
      </div>
    </nav>
  );
};
