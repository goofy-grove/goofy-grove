import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

import { CharacterSchema } from './schema';

export const putAvatar = withValidation(
  CharacterSchema,
  async (uid: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);

    const response = await api.put(`/characters/${uid}/avatar`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });

    return response.data as unknown;
  },
);
