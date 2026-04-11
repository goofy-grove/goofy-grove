import z from 'zod';

import { api } from '../axios';
import { withAuth, withValidation } from '../common';

import { PersonaSchema } from './schema';

export const getAll = withAuth(
  withValidation(z.array(PersonaSchema), async () => {
    const response = await api.get('/persons');

    return response.data as unknown;
  }),
);
