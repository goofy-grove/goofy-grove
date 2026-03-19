import { authState } from './auth-state';

export { withAuth } from './with-auth';

export const updateAuthState = (token: string, exp: number) => {
  authState.token = token;
  authState.exp = exp;
};
