import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { ChatInfoSchema } from './schema';

export const putAvatar = withValidation(
  ChatInfoSchema,
  async (uid: string, file: File) => {
    const formData = new FormData();

    formData.append('file', file);

    const response = await api.put(`/chats/${uid}/avatar`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });

    return response.data as unknown;
  },
);
