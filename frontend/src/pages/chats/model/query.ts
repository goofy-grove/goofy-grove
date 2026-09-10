import { useQuery } from '@tanstack/react-query';

import { api } from '@shared/api';

import { CHATS_QUERY_KEY } from './constants';

export const useChatsQuery = () =>
  useQuery({
    queryKey: [CHATS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.chats.getAll();

      if (response.error) {
        throw new Error('Failed to fetch chats');
      }

      return response.data;
    },
  });
