import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { ChatSchema } from './schema';

export const create = withValidation(ChatSchema, async (name: string) => {
  const response = await api.post('/chats', { name });

  return response.data as unknown;
});
