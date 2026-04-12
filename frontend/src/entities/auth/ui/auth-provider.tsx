import { AuthenticatedUser, useAuthStore } from '@entities/auth/model';
import { AuthContext } from '@entities/auth/model/use-auth';

import { api } from '@shared/api';

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const user = useAuthStore((state) => state.currentUser);
  const loginUser = useAuthStore((state) => state.login);
  const logoutUser = useAuthStore((state) => state.logout);

  const login = async (username: string, password: string) => {
    await api.auth.authorize(username, password);
    const currentUser = await api.users.getMe();

    if (currentUser.error) {
      throw new Error(currentUser.data.reason.join(', '));
    }

    loginUser(
      new AuthenticatedUser(currentUser.data.id, currentUser.data.username),
    );
  };

  const getMe = async () => {
    const currentUser = await api.users.getMe();

    if (currentUser.error) {
      logoutUser();

      throw new Error(currentUser.data.reason.join(', '));
    }

    loginUser(
      new AuthenticatedUser(currentUser.data.id, currentUser.data.username),
    );
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        getMe,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
