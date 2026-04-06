import type { PropsWithChildren, ReactNode } from 'react';

export type ButtonProps = PropsWithChildren<{
  rightIcon?: ReactNode;
  leftIcon?: ReactNode;

  disabled?: boolean;
  className?: string;

  variant?: 'default' | 'ghost';

  onClick?: () => void;
}>;
