import type { PropsWithChildren } from 'react';

export type ModalProps = PropsWithChildren<{
  id: string;
  isOpen?: boolean;

  onClose?: () => void;
  onOpen?: () => void;
}>;
