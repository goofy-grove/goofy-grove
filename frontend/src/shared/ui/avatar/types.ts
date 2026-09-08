import type { ComponentPropsWithoutRef } from 'react';

export type AvatarProps = ComponentPropsWithoutRef<'img'> & {
  size?: 'small' | 'medium' | 'large';
  variant?: 'default' | 'unbordered';
};
