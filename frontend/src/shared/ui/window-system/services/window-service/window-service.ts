import { EventEmitter } from 'events';

import type { OnWindowCloseHandler, OnWindowOpenHandler } from './types';

class WindowService {
  constructor() {
    this.bus = new EventEmitter();
  }

  bus: EventEmitter;

  onWindowOpen<T extends Record<string, unknown>>(
    handler: OnWindowOpenHandler<T>,
  ) {
    this.bus.on('open', handler);

    return () => {
      this.bus.off('open', handler);
    };
  }

  onWindowClose(handler: OnWindowCloseHandler) {
    this.bus.on('close', handler);

    return () => {
      this.bus.off('close', handler);
    };
  }

  openWindow<T>(id: string, props?: T) {
    this.bus.emit('open', id, props);
  }

  closeWindow(id: string) {
    this.bus.emit('close', id);
  }
}

export const windowService = new WindowService();
