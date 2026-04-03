import z from 'zod';

import { api } from '../axios';
import { withAuth, withValidation } from '../common';

import { PersonSchema } from './schema';

export const getAll = withAuth(
  withValidation(z.array(PersonSchema), async () => {
    const response = await api.get('/persons');

    return response.data as unknown;
  }),
);
