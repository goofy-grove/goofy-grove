import type { ComponentType } from 'react';

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

export type WindowComponent = ComponentType<{
  onClose: () => void;
  onMaximize: () => void;
}>;
