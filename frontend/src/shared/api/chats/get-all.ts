import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { ChatSchema } from './schema';

export const getAll = withValidation(ChatSchema.array(), async () => {
  const response = await api.get(`/chats`);

  return response.data as unknown;
});
