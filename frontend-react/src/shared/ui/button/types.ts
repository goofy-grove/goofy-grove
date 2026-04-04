import type { PropsWithChildren, ReactNode } from 'react';

export type ButtonProps = PropsWithChildren<{
  rightIcon?: ReactNode;
  leftIcon?: ReactNode;

  variant?: 'default' | 'ghost';

  onClick?: () => void;
}>;
