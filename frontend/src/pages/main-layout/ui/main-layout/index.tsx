import { Outlet } from '@tanstack/react-router';

import { Sidebar } from '@pages/main-layout/ui/sidebar';

import './styles.scss';

export const MainLayout = () => {
  return (
    <div className="main-layout">
      <Sidebar />

      <div className="main-layout__content">
        <Outlet />
      </div>
    </div>
  );
};
