import type { VirtualElement } from '@floating-ui/react';
import type { ReactNode } from 'react';

export type DropdownProps = {
  children: ReactNode;
  trigger?: ReactNode;
  virtualReference?: VirtualElement | null;
  isOpen: boolean;
  matchTriggerWidth?: boolean;

  onShow?: () => void;
  onHide?: () => void;
};
