import { api } from '../axios';
import { updateAuthState, withValidation } from '../common';
import { AuthResponseSchema } from './schema';

export const refresh = async () => {
  const refreshTokens = withValidation(AuthResponseSchema, async () => {
    const response = await api.post('/auth/refresh');

    return response.data;
  });

  const result = await refreshTokens();

  if (result.error) {
    throw new Error(result.data.reason.join(', '));
  }

  updateAuthState(result.data.token, result.data.exp);
};
