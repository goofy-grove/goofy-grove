import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { CharacterSchema } from './schema';

export const update = withValidation(
  CharacterSchema,
  async (uid: string, name: string, description: string) => {
    const response = await api.patch(`/characters/${uid}`, {
      name,
      description,
    });

    return response.data as unknown;
  },
);
