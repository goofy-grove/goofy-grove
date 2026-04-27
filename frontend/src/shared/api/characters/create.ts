import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { CharacterSchema } from './schema';

export const create = withValidation(
  CharacterSchema,
  async (name: string, description: string) => {
    const response = await api.post('/characters', { name, description });

    return response.data as unknown;
  },
);
