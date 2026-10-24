import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { ChatInfoSchema } from './schema';

import type { UpdateChatPayload } from './types';

export const update = withValidation(
  ChatInfoSchema,
  async (uid: string, payload: UpdateChatPayload) => {
    const response = await api.patch(`/chats/${uid}`, payload);

    return response.data as unknown;
  },
);
