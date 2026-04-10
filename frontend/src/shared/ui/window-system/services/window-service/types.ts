import type { ComponentType } from 'react';
import type { WindowProps } from '../../components';

export type WindowEventType = 'open' | 'close';

export type WindowData<T> = {
  id: string;
  props?: T;
};

export type OnWindowOpenHandler<T extends Record<string, unknown>> = (
  id: string,
  props?: T,
) => void;

export type OnWindowCloseHandler = (id: string) => void;

export type OnWindowMaximizeHandler = (
  id: string,
  isMaximized: boolean,
) => void;

export type WindowComponent = ComponentType<
  Pick<WindowProps, 'onClose' | 'onMaximize' | 'onMinimize'> &
    Record<string, unknown>
>;
