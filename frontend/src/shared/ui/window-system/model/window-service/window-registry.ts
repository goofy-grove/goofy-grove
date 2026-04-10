import type { WindowComponent } from './types';

export class WindowRegistry {
  constructor() {
    this.registry = new Map();
  }

  registry: Map<string, WindowComponent>;

  register(id: string, component: WindowComponent) {
    this.registry.set(id, component);
  }

  get(id: string) {
    return this.registry.get(id);
  }
}

export const windowRegistry = new WindowRegistry();
