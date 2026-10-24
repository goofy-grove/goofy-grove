import { useMutation } from '@tanstack/react-query';
import { v7 } from 'uuid';

import { useAuthStore } from '@entities/auth';

import { api, ApiRequestError, type Chat } from '@shared/api';

import { CHATS_QUERY_KEY } from './constants';

export const useCreateChatMutation = () =>
  useMutation({
    mutationFn: async ({
      name,
      avatarFile,
    }: {
      name: string;
      avatarFile?: File;
    }) => {
      const created = await api.chats.create(name);

      if (created.error) {
        return created;
      }

      if (!avatarFile) {
        return created;
      }

      const chatInfo = await api.chats.putAvatar(created.data.uid, avatarFile);

      if (chatInfo.error) {
        return created;
      }

      created.data.avatar_uid = chatInfo.data.avatar_uid;

      return created;
    },

    onMutate: async ({ name }, context) => {
      await context.client.cancelQueries({ queryKey: [CHATS_QUERY_KEY] });

      const currentUserId = useAuthStore.getState().currentUser?.uid;
      const currentUsername = useAuthStore.getState().currentUser?.username;

      if (!currentUserId || !currentUsername) {
        throw new Error('User is not logged in');
      }

      const chatUid = v7();
      const optimisticResult: Chat = {
        uid: chatUid,
        name,
        created_at: new Date(),
        creator_uid: currentUserId,
        avatar_uid: null,
        members: [
          {
            uid: currentUserId,
            chat_uid: chatUid,
            username: currentUsername,
            joined_at: new Date(),
          },
        ],
        characters: [],
        last_message: null,
      };

      context.client.setQueryData<Chat[]>([CHATS_QUERY_KEY], (old) =>
        old ? [...old, optimisticResult] : [optimisticResult],
      );

      return optimisticResult;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Chat[]>(
        [CHATS_QUERY_KEY],
        (old) =>
          old?.filter((character) => character.uid !== onMutateResult?.uid) ||
          [],
      );
    },

    onSuccess: (result, _, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Chat[]>(
          [CHATS_QUERY_KEY],
          (old) =>
            old?.filter((character) => character.uid !== onMutateResult.uid) ||
            [],
        );

        throw new ApiRequestError(result.data);
      }

      context.client.setQueryData<Chat[]>(
        [CHATS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === onMutateResult.uid ? result.data : character,
          ) || [result.data],
      );
    },
  });

export const useDeleteChatMutation = () =>
  useMutation({
    mutationFn: ({ uid }: { uid: string }) => api.chats.remove(uid),

    onMutate: async ({ uid }, context) => {
      await context.client.cancelQueries({ queryKey: [CHATS_QUERY_KEY] });

      const previousChats =
        context.client.getQueryData<Chat[]>([CHATS_QUERY_KEY]) || [];

      context.client.setQueryData<Chat[]>(
        [CHATS_QUERY_KEY],
        (old) => old?.filter((character) => character.uid !== uid) || [],
      );

      return previousChats;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Chat[]>(
        [CHATS_QUERY_KEY],
        onMutateResult || [],
      );
    },

    onSuccess: (result, __, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Chat[]>(
          [CHATS_QUERY_KEY],
          onMutateResult || [],
        );

        throw new ApiRequestError(result.data);
      }
    },
  });
