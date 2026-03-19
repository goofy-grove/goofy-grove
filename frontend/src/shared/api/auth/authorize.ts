import { api } from '../axios';
import { updateAuthState, withValidation } from '../common';
import { AuthResponseSchema } from './schema';

export const authorize = async (username: string, password: string) => {
  const auth = withValidation(
    AuthResponseSchema,
    (username: string, password: string) =>
      api.post('/auth/login', { password, username }),
  );

  const result = await auth(username, password);

  if (result.error) {
    throw new Error(result.reason.join(', '));
  }

  updateAuthState(result.data.token, result.data.exp);
};
