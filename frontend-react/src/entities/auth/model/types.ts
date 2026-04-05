import type { AuthenticatedUser } from './entity';

export interface AuthStore {
  currentUser: AuthenticatedUser | null;

  login: (user: AuthenticatedUser) => void;
  logout: () => void;
}

export type AuthContextData = {
  user: AuthenticatedUser | null;

  login: (username: string, password: string) => Promise<void>;
  getMe: () => Promise<void>;
};
