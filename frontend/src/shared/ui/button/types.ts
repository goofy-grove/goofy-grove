import type { PropsWithChildren, ReactNode } from 'react';

export type ButtonColor = 'default' | 'error' | 'warning' | 'success' | 'info';

export type ButtonProps = PropsWithChildren<{
  rightIcon?: ReactNode;
  leftIcon?: ReactNode;

  disabled?: boolean;
  className?: string;

  /** Filled surface vs transparent — independent from `color` */
  variant?: 'default' | 'ghost';
  /** Semantic tint; `default` is the current neutral look */
  color?: ButtonColor;

  onClick?: () => void;
}>;
