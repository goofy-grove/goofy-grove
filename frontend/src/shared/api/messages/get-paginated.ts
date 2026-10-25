import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { MessagesPageSchema } from './schema';

import type { GetPaginatedPayload } from './types';

export const getPaginated = withValidation(
  MessagesPageSchema,
  async ({ chatUid, limit, page }: GetPaginatedPayload) => {
    const result = await api.get(`/chats/${chatUid}/messages`, {
      params: { limit, page },
    });

    return result.data as unknown;
  },
);
