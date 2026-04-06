import './styles.scss';

import { IconMenu2, IconUserCircle } from '@tabler/icons-react';
import { Outlet } from '@tanstack/react-router';
import { useEffect, useRef, useState } from 'react';

import { Button, Input, Text } from '../../shared/ui';
import { useAuthStore } from '../../entities/auth';

export const MainLayout = () => {
  const [isSidebarOpened, setIsSidebarOpened] = useState(false);
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

        <Input ref={input} placeholder="Search [Ctrl + K]" />
        <Button leftIcon={<IconUserCircle />}>{user?.username}</Button>
      </div>

      <div className="main-layout__body">
        <div className={`main-layout__sidebar ${!isSidebarOpened ? 'closed' : ''}`}>
          <Text tag="h1">sidebar</Text>
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
