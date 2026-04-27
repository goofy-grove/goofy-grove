import z from 'zod';

import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { CharacterSchema } from './schema';

export const getAll = withValidation(z.array(CharacterSchema), async () => {
  const response = await api.get('/characters');

  return response.data as unknown;
});
