import { useQuery } from '@tanstack/react-query';

import { api, socket } from '@shared/api';
import type { Persona } from '@shared/api';
import { queryClient } from '@shared/lib';

import { PERSONAS_QUERY_KEY } from './constants';

import type { PersonaDeletedEventData } from './types';

export const usePersonasQuery = () =>
  useQuery({
    queryKey: [PERSONAS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.personas.getAll();

      if (response.error) {
        throw new Error('Failed to fetch personas');
      }

      return response.data;
    },
  });

socket.on('persona:created', (persona: Persona) => {
  queryClient.setQueryData<Persona[]>([PERSONAS_QUERY_KEY], (old) =>
    old ? [...old, persona] : [persona],
  );
});

socket.on('persona:updated', (persona: Persona) => {
  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) =>
      old?.map((item) => (item.uid !== persona.uid ? item : persona)) || [],
  );
});

socket.on('persona:deleted', (payload: PersonaDeletedEventData) => {
  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) => old?.filter((item) => item.uid !== payload.uid) || [],
  );
});
