import './styles.scss';

import {
  IconHome,
  IconMenu2,
  IconSettings,
  IconUser,
  IconUserCircle,
} from '@tabler/icons-react';
import { Outlet } from '@tanstack/react-router';
import { useEffect, useRef, useState } from 'react';
import { useMediaQuery } from 'react-responsive';

import { Button, Input } from '../../shared/ui';
import { useAuthStore } from '../../entities/auth';

export const MainLayout = () => {
  const [isSidebarOpened, setIsSidebarOpened] = useState(true);
  // FIXME: make it as independent hook
  const isSemiMobile = useMediaQuery({ query: '(max-width: 1024px)' });
  const input = useRef<HTMLInputElement>(null);
  const user = useAuthStore((state) => state.currentUser);

  useEffect(() => {
    const handleCtrlK = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === 'k') {
        input.current?.focus();
      }
    };

    document.addEventListener('keydown', handleCtrlK);

    return () => document.removeEventListener('keydown', handleCtrlK);
  }, []);

  return (
    <div className="main-layout">
      <div className="main-layout__header">
        <Button
          className="main-layout__header__menu-button"
          variant="ghost"
          leftIcon={<IconMenu2 />}
          onClick={() => setIsSidebarOpened(!isSidebarOpened)}
        />

        <div className="main-layout__header__right-actions">
          <Input ref={input} placeholder="Search [Ctrl + K]" />
          <Button leftIcon={<IconUserCircle />}>{user?.username}</Button>
        </div>
      </div>

      <div className="main-layout__body">
        <div
          className={`main-layout__sidebar ${!isSidebarOpened ? 'collapsed' : ''}`}
        >
          <Button variant="ghost" leftIcon={<IconHome />}>
            {isSidebarOpened || isSemiMobile ? 'Home' : ''}
          </Button>
          <Button variant="ghost" leftIcon={<IconUser />}>
            {isSidebarOpened || isSemiMobile ? 'Personas' : ''}
          </Button>
          <Button variant="ghost" leftIcon={<IconSettings />}>
            {isSidebarOpened || isSemiMobile ? 'Settings' : ''}
          </Button>
        </div>

        <div className="main-layout__content-wrapper">
          <div className="main-layout__content">
            <Outlet />
          </div>
        </div>
      </div>
    </div>
  );
};
