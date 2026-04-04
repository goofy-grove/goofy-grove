import type {
  ComponentPropsWithoutRef,
  PropsWithChildren,
  RefObject,
} from 'react';

export type CardProps = ComponentPropsWithoutRef<'div'> &
  PropsWithChildren<{
    closable?: boolean;

    headerRef?: RefObject<HTMLDivElement | null>;
    ref?: RefObject<HTMLDivElement | null>;

    onClose?: () => void;
  }>;
