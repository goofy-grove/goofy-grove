import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { CharacterSchema } from './schema';

import type { UpdateCharacterPayload } from './types';

export const update = withValidation(
  CharacterSchema,
  async (uid: string, payload: UpdateCharacterPayload) => {
    const response = await api.patch(`/characters/${uid}`, payload);

    return response.data as unknown;
  },
);
