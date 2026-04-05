import { createElement, type FC } from 'react';

import type { TextProps } from './types';

import './styles.scss';

export const Text: FC<TextProps> = ({ children, variant, tag = 'p' }) =>
  createElement(tag, { className: `text ${variant ?? ''}` }, children);
