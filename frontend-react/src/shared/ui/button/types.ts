import type { PropsWithChildren, ReactNode } from 'react';

export type ButtonProps = PropsWithChildren<{
  rightIcon?: ReactNode;
  leftIcon?: ReactNode;

  disabled?: boolean;

  variant?: 'default' | 'ghost';

  onClick?: () => void;
}>;
