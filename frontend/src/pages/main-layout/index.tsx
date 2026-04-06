import { Outlet } from '@tanstack/react-router';

import './styles.scss';
import { Text } from '../../shared/ui';

export const MainLayout = () => {
  return (
    <div className="main-layout">
      <div className="main-layout__header"></div>

      <div className="main-layout__body">
        <div className="main-layout__content">
          <Outlet />
        </div>

        <div className="main-layout__sidebar">
          <Text tag="h1">sidebar</Text>
        </div>
      </div>

      <div className="main-layout__footer"></div>
    </div>
  );
};
