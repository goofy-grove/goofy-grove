import { useMutation } from '@tanstack/react-query';
import { v4 } from 'uuid';

import { useAuthStore } from '@entities/auth/@x/persona';

import { api } from '@shared/api';

import { PERSONAS_QUERY_KEY } from './constants';
import { Persona } from './entity';

export const useCreatePersonaMutation = () =>
  useMutation({
    mutationFn: ({
      name,
      description,
    }: {
      name: string;
      description: string;
    }) => api.personas.create(name, description),

    onMutate: async ({ name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [PERSONAS_QUERY_KEY] });

      const currentUserId = useAuthStore.getState().currentUser?.id;

      if (!currentUserId) {
        throw new Error('User is not logged in');
      }

      const optimisticResult = new Persona(
        v4(),
        name,
        description,
        currentUserId,
      );

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

        throw new Error('Failed to create persona');
      }

      const newPersona = new Persona(
        result.data.uid,
        result.data.name,
        result.data.description,
        result.data.creator_uid,
      );

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === onMutateResult.uid ? newPersona : persona,
          ) || [newPersona],
      );
    },
  });

export const useUpdatePersonaMutation = () =>
  useMutation({
    mutationFn: ({
      uid,
      name,
      description,
    }: {
      uid: string;
      name: string;
      description: string;
    }) => api.personas.update(uid, name, description),

    onMutate: async ({ uid, name, description }, context) => {
      await context.client.cancelQueries({ queryKey: [PERSONAS_QUERY_KEY] });

      const previousPersonas =
        context.client.getQueryData<Persona[]>([PERSONAS_QUERY_KEY]) || [];

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === uid
              ? new Persona(persona.uid, name, description, persona.creatorUid)
              : persona,
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

        throw new Error('Failed to update persona');
      }

      const updatedPersona = new Persona(
        result.data.uid,
        result.data.name,
        result.data.description,
        result.data.creator_uid,
      );

      context.client.setQueryData<Persona[]>(
        [PERSONAS_QUERY_KEY],
        (old) =>
          old?.map((persona) =>
            persona.uid === uid ? updatedPersona : persona,
          ) || [updatedPersona],
      );
    },
  });
