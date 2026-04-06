import type { PropsWithChildren } from 'react';
import type { Props as RndProps } from 'react-rnd';
import type { CardProps } from '../../card';

export type ModalProps = Pick<CardProps, 'withoutHeader'> &
  Pick<RndProps, 'bounds'> &
  PropsWithChildren<{
    id: string;
    isOpen?: boolean;

    onClose?: () => void;
    onOpen?: () => void;
  }>;
