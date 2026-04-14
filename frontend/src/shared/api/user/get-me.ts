import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { UserResponseSchema } from './schema';

export const getMe = withValidation(UserResponseSchema, async () => {
  const response = await api.get('/users/me');

  return response.data as unknown;
});
