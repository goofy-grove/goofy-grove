import { useMutation } from '@tanstack/react-query';
import { v4 } from 'uuid';

import { useAuthStore } from '@entities/auth';

import { api, ApiRequestError } from '@shared/api';
import type { Character } from '@shared/api';

import { CHARACTERS_QUERY_KEY } from './constants';

export const useCreateCharacterMutation = () =>
  useMutation({
    mutationFn: async ({
      name,
      description,
      avatarFile,
    }: {
      name: string;
      description: string;
      avatarFile?: File | null;
    }) => {
      const created = await api.characters.create(name, description);

      if (created.error) {
        return created;
      }

      if (!avatarFile) {
        return created;
      }

      return api.characters.putAvatar(created.data.uid, avatarFile);
    },

    onMutate: async ({ name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [CHARACTERS_QUERY_KEY] });

      const currentUserId = useAuthStore.getState().currentUser?.uid;

      if (!currentUserId) {
        throw new Error('User is not logged in');
      }

      const optimisticResult: Character = {
        uid: v4(),
        name,
        description,
        creator_uid: currentUserId,
        avatar_uid: null,
      };

      context.client.setQueryData<Character[]>([CHARACTERS_QUERY_KEY], (old) =>
        old ? [...old, optimisticResult] : [optimisticResult],
      );

      return optimisticResult;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.filter((character) => character.uid !== onMutateResult?.uid) ||
          [],
      );
    },

    onSuccess: (result, _, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Character[]>(
          [CHARACTERS_QUERY_KEY],
          (old) =>
            old?.filter((character) => character.uid !== onMutateResult.uid) ||
            [],
        );

        throw new ApiRequestError(result.data);
      }

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === onMutateResult.uid ? result.data : character,
          ) || [result.data],
      );
    },
  });

export const useUpdateCharacterMutation = () =>
  useMutation({
    mutationFn: async ({
      uid,
      name,
      description,
      avatarFile,
    }: {
      uid: string;
      name: string;
      description: string;
      avatarFile?: File | null;
    }) => {
      const updated = await api.characters.update(uid, {
        name,
        description,
      });

      if (updated.error) {
        return updated;
      }

      if (!avatarFile) {
        return updated;
      }

      return api.characters.putAvatar(uid, avatarFile);
    },

    onMutate: async ({ uid, name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [CHARACTERS_QUERY_KEY] });

      const previousCharacters =
        context.client.getQueryData<Character[]>([CHARACTERS_QUERY_KEY]) || [];

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === uid
              ? { ...character, name, description }
              : character,
          ) || [],
      );

      return previousCharacters;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        onMutateResult || [],
      );
    },

    onSuccess: (result, { uid }, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Character[]>(
          [CHARACTERS_QUERY_KEY],
          onMutateResult || [],
        );

        throw new ApiRequestError(result.data);
      }

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === uid ? result.data : character,
          ) || [result.data],
      );
    },
  });

export const useDeleteCharacterMutation = () =>
  useMutation({
    mutationFn: ({ uid }: { uid: string }) => api.characters.remove(uid),

    onMutate: async ({ uid }, context) => {
      await context.client.cancelQueries({ queryKey: [CHARACTERS_QUERY_KEY] });

      const previousCharacters =
        context.client.getQueryData<Character[]>([CHARACTERS_QUERY_KEY]) || [];

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) => old?.filter((character) => character.uid !== uid) || [],
      );

      return previousCharacters;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        onMutateResult || [],
      );
    },

    onSuccess: (result, __, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Character[]>(
          [CHARACTERS_QUERY_KEY],
          onMutateResult || [],
        );

        throw new ApiRequestError(result.data);
      }
    },
  });
