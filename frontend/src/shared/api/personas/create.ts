import { api } from '@shared/api/axios';
import { withAuth, withValidation } from '@shared/api/common';

import { PersonaSchema } from './schema';

export const create = withAuth(
  withValidation(PersonaSchema, async (name: string, description: string) => {
    const response = await api.post('/personas', { name, description });

    return response.data as unknown;
  }),
);
