import type { User } from '@shared/api';

export interface AuthStore {
  currentUser: User | null;

  login: (user: User) => void;
  logout: () => void;
}

export type AuthContextData = {
  user: User | null;

  login: (username: string, password: string) => Promise<void>;
  getMe: () => Promise<void>;
  logout: () => Promise<void>;
};
