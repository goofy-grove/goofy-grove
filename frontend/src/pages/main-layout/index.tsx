import { Outlet } from '@tanstack/react-router';

import { useBreakpoints } from '../../shared/ui';

import { Menu } from './ui';

import './styles.scss';

export const MainLayout = () => {
  const { isTabletSm } = useBreakpoints();

  return (
    <div className="main-layout">
      <div className="main-layout__content">
        <Outlet />
      </div>

      <div className="main-layout__footer">
        <div className="main-layout__footer__content">
          <Menu isCollapsed={isTabletSm} />
        </div>
      </div>
    </div>
  );
};
