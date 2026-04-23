import z from 'zod';

import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

export const remove = withValidation(z.object({}), async (uid: string) => {
  const response = await api.delete(`/personas/${uid}`);

  return response.data as unknown;
});
