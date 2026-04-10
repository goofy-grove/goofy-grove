import type { ComponentType } from 'react';
import type { WindowProps } from '../../components';

export type WindowComponent = ComponentType<
  Pick<WindowProps, 'onClose' | 'onMaximize' | 'onMinimize'> &
    Record<string, unknown>
>;
