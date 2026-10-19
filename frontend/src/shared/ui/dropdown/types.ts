import type { VirtualElement } from '@floating-ui/react';
import type { HTMLAttributes, ReactNode } from 'react';

export type DropdownProps = {
  children: ReactNode;
  trigger?: ReactNode;
  virtualReference?: VirtualElement | null;
  isOpen: boolean;
  matchTriggerWidth?: boolean;
  contentClass?: HTMLAttributes<HTMLDivElement>['className'];

  onShow?: () => void;
  onHide?: () => void;
};
