import type { PropsWithChildren } from 'react';
import type { Props as RndProps } from 'react-rnd';
import type { CardProps } from '../../../card';

export type WindowProps = Pick<CardProps, 'withoutHeader'> &
  Pick<RndProps, 'bounds'> &
  PropsWithChildren<{
    id: string;

    title?: string;
    isMaximized?: boolean;

    onClose?: () => void;
    onMaximize?: () => void;
    onMinimize?: () => void;
  }>;
