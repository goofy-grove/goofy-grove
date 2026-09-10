import { create } from 'zustand';

import type { AuthStore } from './types';

export const useAuthStore = create<AuthStore>((set) => ({
  currentUser: null,

  login: (user) => set({ currentUser: user }),
  logout: () => set({ currentUser: null }),
}));
