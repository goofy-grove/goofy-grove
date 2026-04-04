import type { ReactNode } from 'react';

export type DropdownProps = {
  children: ReactNode;
  trigger: ReactNode;
  isOpen?: boolean;

  onShow?: () => void;
  onHide?: () => void;
};
