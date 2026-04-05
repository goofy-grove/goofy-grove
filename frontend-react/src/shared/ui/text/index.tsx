import { createElement, type FC } from 'react';

import type { TextProps } from './types';

import './styles.scss';

export const Text: FC<TextProps> = ({
  children,
  variant,
  className,
  tag = 'p',
  ...rest
}) =>
  createElement(
    tag,
    { className: `text ${variant ?? ''} ${className ?? ''}`, ...rest },
    children,
  );
