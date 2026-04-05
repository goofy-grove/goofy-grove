import type {
  ComponentPropsWithoutRef,
  PropsWithChildren,
  ReactNode,
  RefObject,
} from 'react';

export type CardProps = ComponentPropsWithoutRef<'div'> &
  PropsWithChildren<{
    closable?: boolean;

    actions?: ReactNode;

    headerRef?: RefObject<HTMLDivElement | null>;
    ref?: RefObject<HTMLDivElement | null>;

    onClose?: () => void;
  }>;
