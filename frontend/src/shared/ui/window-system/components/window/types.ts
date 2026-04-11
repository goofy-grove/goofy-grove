import type { PropsWithChildren } from 'react';
import type { Props as RndProps } from 'react-rnd';
import type { CardProps } from '../../../card';

export type WindowProps = Pick<CardProps, 'withoutHeader'> &
  Pick<RndProps, 'bounds'> &
  PropsWithChildren<{
    title?: string;
  }>;
