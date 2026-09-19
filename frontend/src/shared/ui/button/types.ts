import type { ComponentPropsWithRef, ReactNode } from 'react';

export type ButtonColor = 'default' | 'error' | 'warning' | 'success' | 'info';

export type ButtonProps = Omit<ComponentPropsWithRef<'button'>, 'color'> & {
  rightIcon?: ReactNode;
  leftIcon?: ReactNode;
  variant?: 'default' | 'ghost';
  color?: ButtonColor;
  size?: 'default' | 'compact';
};
