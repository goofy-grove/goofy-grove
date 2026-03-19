import { api } from '../axios';
import { withAuth, withValidation } from '../common';
import { UserResponseSchema } from './schema';

export const getMe = withAuth(
  withValidation(UserResponseSchema, () => api.get('/users/me')),
);
