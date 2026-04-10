import type { ComponentType } from 'react';

export type WindowType = {
  id: string;
  component: ComponentType;
};

export type WindowRegistry = Record<string, WindowType>;

export type WindowState = {
  id: string;
  isMaximized: boolean;
  lastInteraction: number;
  props?: Record<string, unknown>;
};

export type WindowContextType = {
  activeWindows: WindowState[];
};
