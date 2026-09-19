import type { ComponentPropsWithoutRef, ReactNode } from 'react';

export type AvatarProps = ComponentPropsWithoutRef<'img'> & {
  fallback?: ReactNode;
  size?: 'small' | 'medium' | 'large';
  variant?: 'default' | 'unbordered';
};
