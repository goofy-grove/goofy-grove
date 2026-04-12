import z from 'zod';

import { api } from '@shared/api/axios';
import { withAuth, withValidation } from '@shared/api/common';

import { PersonaSchema } from './schema';

export const getAll = withAuth(
  withValidation(z.array(PersonaSchema), async () => {
    const response = await api.get('/persons');

    return response.data as unknown;
  }),
);
