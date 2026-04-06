import { create } from 'zustand';

import type { AuthenticatedUser } from './entity';
import type { AuthStore } from './types';

export const useAuthStore = create<AuthStore>((set) => ({
  currentUser: null,

  login: (user: AuthenticatedUser) => set({ currentUser: user }),
  logout: () => set({ currentUser: null }),
}));
