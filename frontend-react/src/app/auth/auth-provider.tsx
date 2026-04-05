import { useState } from 'react';

import { api } from '../../shared/api';
import { type User } from '../../entities/users';

import { AuthContext } from './use-auth';

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [user, setUser] = useState<User | undefined>();
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const login = async (username: string, password: string) => {
    try {
      await api.auth.authorize(username, password);
      const currentUser = await api.users.getMe();

      if (currentUser.error) {
        throw new Error(currentUser.data.reason.join(', '));
      }

      setUser(currentUser.data);
      setIsAuthenticated(true);
    } catch {
      throw new Error('Invalid login or password');
    }
  };

  const getMe = async () => {
    const currentUser = await api.users.getMe();

    if (currentUser.error) {
      setUser(undefined);
      setIsAuthenticated(false);

      throw new Error(currentUser.data.reason.join(', '));
    }

    setUser(currentUser.data);
    setIsAuthenticated(true);

    return currentUser.data;
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        isAuthenticated,
        login,
        getMe,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
