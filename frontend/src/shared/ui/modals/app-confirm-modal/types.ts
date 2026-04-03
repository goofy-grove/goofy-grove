import type { AppModalEmits, AppModalProps } from '../app-modal';

export interface AppConfirmModalProps extends AppModalProps {
  showCancel?: boolean;
  showConfirm?: boolean;
  isLoading?: boolean;
}

export interface AppConfirmModalEmits extends AppModalEmits {
  (e: 'confirm'): void;
}
