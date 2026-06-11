import { useMutation } from '@tanstack/react-query';
import { v4 } from 'uuid';

import { useAuthStore } from '@entities/auth/@x/character';

import { api } from '@shared/api';

import { CHARACTERS_QUERY_KEY } from './constants';
import { Character } from './entity';

export const useCreateCharacterMutation = () =>
  useMutation({
    mutationFn: ({
      name,
      description,
    }: {
      name: string;
      description: string;
    }) => api.characters.create(name, description),

    onMutate: async ({ name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [CHARACTERS_QUERY_KEY] });

      const currentUserId = useAuthStore.getState().currentUser?.uid;

      if (!currentUserId) {
        throw new Error('User is not logged in');
      }

      const optimisticResult = new Character(
        v4(),
        name,
        description,
        currentUserId,
      );

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

        throw new Error('Failed to create character');
      }

      const newCharacter = new Character(
        result.data.uid,
        result.data.name,
        result.data.description,
        result.data.creator_uid,
      );

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === onMutateResult.uid ? newCharacter : character,
          ) || [newCharacter],
      );
    },
  });

export const useUpdateCharacterMutation = () =>
  useMutation({
    mutationFn: ({
      uid,
      name,
      description,
    }: {
      uid: string;
      name: string;
      description: string;
    }) => api.characters.update(uid, name, description),

    onMutate: async ({ uid, name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [CHARACTERS_QUERY_KEY] });

      const previousCharacters =
        context.client.getQueryData<Character[]>([CHARACTERS_QUERY_KEY]) || [];

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === uid
              ? new Character(
                  character.uid,
                  name,
                  description,
                  character.creatorUid,
                )
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

        throw new Error('Failed to update character');
      }

      const updatedCharacter = new Character(
        result.data.uid,
        result.data.name,
        result.data.description,
        result.data.creator_uid,
      );

      context.client.setQueryData<Character[]>(
        [CHARACTERS_QUERY_KEY],
        (old) =>
          old?.map((character) =>
            character.uid === uid ? updatedCharacter : character,
          ) || [updatedCharacter],
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

        throw new Error('Failed to delete character');
      }
    },
  });
