import type { WindowInstance } from '../types';

export type WinowState = {
  windows: WindowInstance[];

  openWindow: (type: string, props?: Record<string, unknown>) => string;
  closeWindow: (instanceId: string) => void;
  maximizeWindow: (instanceId: string) => void;
  minimizeWindow: (instanceId: string) => void;
};
