import type { ButtonColor } from '@shared/ui';

import type { ReactNode } from 'react';

export type ConfirmModalProps = {
  isOpen: boolean;
  title?: ReactNode;
  message: ReactNode;
  confirmLabel: ReactNode;
  cancelLabel: ReactNode;
  confirmColor?: ButtonColor;
  isPending?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
};
