import type { CardProps } from '@shared/ui/card';

import type { PropsWithChildren } from 'react';
import type { Props as RndProps } from 'react-rnd';

export type WindowProps = Pick<CardProps, 'withoutHeader'> &
  Pick<RndProps, 'bounds'> &
  PropsWithChildren<{
    title?: string;

    defaultWidth?: string | number;
    defaultHeight?: string | number;

    defaultX?: number;
    defaultY?: number;
  }>;
