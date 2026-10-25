import { useInfiniteQuery, useQuery } from '@tanstack/react-query';

import { api } from '@shared/api';

import {
  CHAT_PAGE_SIZE,
  CHATS_QUERY_KEY,
  MESSAGE_QUERY_KEY,
} from './constants';

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

export const useMessagesQuery = (chatUid: string) =>
  useInfiniteQuery({
    queryKey: [MESSAGE_QUERY_KEY, chatUid, { paginated: true }],
    queryFn: async ({ pageParam }: { pageParam?: string }) => {
      const response = await api.messages.getPaginated({
        chatUid,
        page: pageParam,
        limit: CHAT_PAGE_SIZE,
      });

      if (response.error) {
        throw new Error('Failed to fetch chats');
      }

      return response.data;
    },
    initialPageParam: undefined,
    getNextPageParam: (data) => data.next_page,
  });
