import z from 'zod';

import { api } from '@shared/api/axios';
import { withValidation } from '@shared/api/common';

const FileUploadSchema = z.object({
  uid: z.string(),
});

export const uploadAvatar = withValidation(
  FileUploadSchema,
  async (uid: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);

    const response = await api.post(`/personas/${uid}/avatar`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });

    return response.data as unknown;
  },
);
