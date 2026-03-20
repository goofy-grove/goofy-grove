import { api } from '../axios';
import { updateAuthState, withValidation } from '../common';
import { AuthResponseSchema } from './schema';

export const authorize = async (username: string, password: string) => {
  const auth = withValidation(
    AuthResponseSchema,
    async (username: string, password: string) => {
      const response = await api.post('/auth/login', { password, username });

      return response.data;
    }
  );

  const result = await auth(username, password);

  if (result.error) {
    throw new Error(result.data.reason.join(', '));
  }

  updateAuthState(result.data.token, result.data.exp);
};
