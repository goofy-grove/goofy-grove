import { IconLoader2 } from '@tabler/icons-react';

import type { FC } from 'react';
import type { IconLoaderProps } from './types';

import './styles.scss';

export const IconLoader: FC<IconLoaderProps> = ({
  isAnimated,
  className,
  ...rest
}) => (
  <IconLoader2
    className={`${className ?? ''} ${isAnimated ? 'animate-spin' : ''}`}
    {...rest}
  />
);
