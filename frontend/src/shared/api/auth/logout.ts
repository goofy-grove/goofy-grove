import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';
import { updateSocketAuth } from '@shared/api/socket';

import { clearAuthState } from './refresh';
import { LogoutResponseSchema } from './schema';

export const logout = async () => {
  const request = withValidation(LogoutResponseSchema, async () => {
    const response = await api.post('/auth/logout', {}, { skipAuth: true });

    return response.data as unknown;
  });

  try {
    await request();
  } finally {
    clearAuthState();
    updateSocketAuth(null);
  }
};
