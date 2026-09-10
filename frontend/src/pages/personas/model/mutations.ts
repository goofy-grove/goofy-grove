import { useMutation } from '@tanstack/react-query';
import { v4 } from 'uuid';

import { useAuthStore } from '@entities/auth';

import { api, ApiRequestError } from '@shared/api';
import type { Persona } from '@shared/api';

import { PERSONAS_QUERY_KEY } from './constants';

export const useCreatePersonaMutation = () =>
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
      const created = await api.personas.create(name, description);

      if (created.error) {
        return created;
      }

      if (!avatarFile) {
        return created;
      }

      return api.personas.putAvatar(created.data.uid, avatarFile);
    },

    onMutate: async ({ name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [PERSONAS_QUERY_KEY] });

      const currentUserId = useAuthStore.getState().currentUser?.uid;

      if (!currentUserId) {
        throw new Error('User is not logged in');
      }

      const optimisticResult: Persona = {
        uid: v4(),
        name,
        description,
        creator_uid: currentUserId,
        avatar_uid: null,
      };

      context.client.setQueryData<Persona[]>([PERSONAS_QUERY_KEY], (old) =>
        old ? [...old, optimisticResult] : [optimisticResult],
      );

      return optimisticResult;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.filter((persona) => persona.uid !== onMutateResult?.uid) || [],
      );
    },

    onSuccess: (result, _, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Persona[]>(
          [PERSONAS_QUERY_KEY],
          (old) =>
            old?.filter((persona) => persona.uid !== onMutateResult.uid) || [],
        );

        throw new ApiRequestError(result.data);
      }

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === onMutateResult.uid ? result.data : persona,
          ) || [result.data],
      );
    },
  });

export const useUpdatePersonaMutation = () =>
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
      const updated = await api.personas.update(uid, {
        name,
        description,
      });

      if (updated.error) {
        return updated;
      }

      if (!avatarFile) {
        return updated;
      }

      return api.personas.putAvatar(uid, avatarFile);
    },

    onMutate: async ({ uid, name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [PERSONAS_QUERY_KEY] });

      const previousPersonas =
        context.client.getQueryData<Persona[]>([PERSONAS_QUERY_KEY]) || [];

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === uid ? { ...persona, name, description } : persona,
          ) || [],
      );

      return previousPersonas;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        onMutateResult || [],
      );
    },

    onSuccess: (result, { uid }, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Persona[]>(
          [PERSONAS_QUERY_KEY],
          onMutateResult || [],
        );

        throw new ApiRequestError(result.data);
      }

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === uid ? result.data : persona,
          ) || [result.data],
      );
    },
  });

export const useDeletePersonaMutation = () =>
  useMutation({
    mutationFn: ({ uid }: { uid: string }) => api.personas.remove(uid),

    onMutate: async ({ uid }, context) => {
      await context.client.cancelQueries({ queryKey: [PERSONAS_QUERY_KEY] });

      const previousPersonas =
        context.client.getQueryData<Persona[]>([PERSONAS_QUERY_KEY]) || [];

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) => old?.filter((persona) => persona.uid !== uid) || [],
      );

      return previousPersonas;
    },

    onError: (_, __, onMutateResult, context) => {
      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        onMutateResult || [],
      );
    },

    onSuccess: (result, __, onMutateResult, context) => {
      if (result.error) {
        context.client.setQueryData<Persona[]>(
          [PERSONAS_QUERY_KEY],
          onMutateResult || [],
        );

        throw new ApiRequestError(result.data);
      }
    },
  });
