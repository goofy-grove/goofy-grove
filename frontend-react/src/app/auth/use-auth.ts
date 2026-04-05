import { createContext, useContext } from 'react';

import type { AuthContextData } from './types';

export const AuthContext = createContext<AuthContextData | null>(null);

export const useAuth = () => {
  const context = useContext(AuthContext);

  if (!context) {
    throw new Error('useAuth must be used within a AuthProvider');
  }

  return context;
};
