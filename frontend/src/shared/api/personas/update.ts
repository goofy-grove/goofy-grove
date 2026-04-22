import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { PersonaSchema } from './schema';

export const update = withValidation(
  PersonaSchema,
  async (uid: string, name: string, description: string) => {
    const response = await api.patch(`/personas/${uid}`, { name, description });

    return response.data as unknown;
  },
);
