import { api, setUpAuthInterceptor } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { AuthResponseSchema } from './schema';

const authState = {
  exp: 0,
  token: '',
};

const MILLISECONDS_IN_SECOND = 1_000;

export const updateAuthState = (token: string, exp: number) => {
  authState.token = token;
  authState.exp = exp;
};

const refresh = async () => {
  const refreshTokens = withValidation(AuthResponseSchema, async () => {
    const response = await api.post('/auth/refresh', {}, { skipAuth: true });

    return response.data as unknown;
  });

  const result = await refreshTokens();

  if (result.error) {
    throw new Error(result.data.reason.join(', '));
  }

  updateAuthState(result.data.token, result.data.exp);
};

setUpAuthInterceptor(() => {
  if (!authState.token || Date.now() > authState.exp * MILLISECONDS_IN_SECOND) {
    return null;
  }

  return authState.token;
}, refresh);
