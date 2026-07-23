import {
  AuthContext,
  AuthenticatedUser,
  useAuthStore,
} from '@entities/auth/model';

import { api } from '@shared/api';

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const user = useAuthStore((state) => state.currentUser);
  const loginUser = useAuthStore((state) => state.login);
  const logoutUser = useAuthStore((state) => state.logout);

  const login = async (username: string, password: string) => {
    await api.auth.authenticate(username, password);
    const currentUser = await api.users.getMe();

    if (currentUser.error) {
      throw new Error(currentUser.data.code);
    }

    loginUser(
      new AuthenticatedUser(currentUser.data.uid, currentUser.data.username),
    );
  };

  const getMe = async () => {
    const currentUser = await api.users.getMe();

    if (currentUser.error) {
      logoutUser();

      throw new Error(currentUser.data.code);
    }

    loginUser(
      new AuthenticatedUser(currentUser.data.uid, currentUser.data.username),
    );
  };

  const logout = async () => {
    try {
      await api.auth.logout();
    } finally {
      logoutUser();
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        getMe,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
