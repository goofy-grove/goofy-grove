import z from 'zod';

import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { PersonaSchema } from './schema';

export const getAll = withValidation(z.array(PersonaSchema), async () => {
  const response = await api.get('/personas');

  return response.data as unknown;
});
