import { api } from '../axios';
import { withAuth, withValidation } from '../common';

import { PersonaSchema } from './schema';

export const create = withAuth(
  withValidation(PersonaSchema, async (name: string, description: string) => {
    const response = await api.post('/persons', { name, description });

    return response.data as unknown;
  }),
);
