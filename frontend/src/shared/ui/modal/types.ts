import type { PropsWithChildren, ReactNode } from 'react';

export type ModalProps = PropsWithChildren<{
  isOpen: boolean;
  title?: ReactNode;
  className?: string;
  onClose?: () => void;
}>;
