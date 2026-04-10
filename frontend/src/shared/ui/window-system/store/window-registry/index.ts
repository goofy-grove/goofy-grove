import { create } from 'zustand';

import type { WindowComponent } from './types';

type WindowConfiguration = {
  component: WindowComponent;
};

interface WindowRegistryState {
  registry: Map<string, WindowConfiguration>;

  register: (id: string, component: WindowComponent) => void;
  get: (id: string) => WindowConfiguration | undefined;
  unregister: (id: string) => void;
}

export const useWindowRegistryStore = create<WindowRegistryState>(
  (set, get) => ({
    registry: new Map(),

    register: (id: string, component: WindowComponent) =>
      set((state) => {
        const newRegistry = new Map(state.registry);

        newRegistry.set(id, { component });

        return { registry: newRegistry };
      }),

    get: (id: string) => get().registry.get(id),

    unregister: (id: string) =>
      set((state) => {
        const newRegistry = new Map(state.registry);

        newRegistry.delete(id);

        return { registry: newRegistry };
      }),
  }),
);

export const registerWindow = (id: string, component: WindowComponent) =>
  useWindowRegistryStore.getState().register(id, component);

export const getWindow = (id: string): WindowComponent | undefined =>
  useWindowRegistryStore.getState().get(id)?.component;

export const unregisterWindow = (id: string) =>
  useWindowRegistryStore.getState().unregister(id);
