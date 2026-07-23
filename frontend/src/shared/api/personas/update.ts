import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { PersonaSchema } from './schema';

import type { UpdatePersonaPayload } from './types';

export const update = withValidation(
  PersonaSchema,
  async (uid: string, payload: UpdatePersonaPayload) => {
    const response = await api.patch(`/personas/${uid}`, payload);

    return response.data as unknown;
  },
);
