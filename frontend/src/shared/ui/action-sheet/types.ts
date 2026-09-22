import type { ButtonProps } from '@shared/ui/button';

export type ActionSheetItem = Pick<ButtonProps, 'color'> & {
  title: string;
  Icon?: React.ReactNode;
  onClick?: () => void;
};

export type ActionSheetProps = {
  isOpened: boolean;
  items: ActionSheetItem[];
  closeTitle?: string;
  showClose?: boolean;

  onClose: () => void;
};
