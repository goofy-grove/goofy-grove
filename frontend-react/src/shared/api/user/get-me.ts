import { api } from '../axios';
import { withAuth, withValidation } from '../common';

import { UserResponseSchema } from './schema';

export const getMe = withAuth(
  withValidation(UserResponseSchema, async () => {
    const response = await api.get('/users/me');

    return response.data as unknown;
  }),
);
