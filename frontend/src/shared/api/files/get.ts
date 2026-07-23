import { api } from '@shared/api/axios';

export const getBlob = async (uid: string): Promise<Blob> => {
  const response = await api.get(`/files/${uid}`, {
    responseType: 'blob',
  });

  return response.data as Blob;
};
