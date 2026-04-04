import type { ComponentPropsWithoutRef, PropsWithChildren } from 'react';

export type CardProps = PropsWithChildren<{
  closable?: boolean;

  onClose?: () => void;
}> &
  ComponentPropsWithoutRef<'div'>;
