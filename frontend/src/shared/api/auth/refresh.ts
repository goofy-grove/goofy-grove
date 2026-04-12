import { api } from '@shared/api/axios';
import { updateAuthState, withValidation } from '@shared/api/common';

import { AuthResponseSchema } from './schema';

export const refresh = async () => {
  const refreshTokens = withValidation(AuthResponseSchema, async () => {
    const response = await api.post('/auth/refresh');

    return response.data as unknown;
  });

  const result = await refreshTokens();

  if (result.error) {
    throw new Error(result.data.reason.join(', '));
  }

  updateAuthState(result.data.token, result.data.exp);
};
