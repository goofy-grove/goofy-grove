import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { UserSchema } from './schema';

export const getMe = withValidation(UserSchema, async () => {
  const response = await api.get('/users/me');

  return response.data as unknown;
});
