import type { User } from '../../entities/users';

export type AuthContextData = {
  user?: User;
  isAuthenticated: boolean;

  login: (username: string, password: string) => Promise<void>;
  getMe: () => Promise<User>;
};
