import { useQuery } from '@tanstack/react-query';

import { api, socket } from '@shared/api';
import { queryClient } from '@shared/lib';

import { PERSONAS_QUERY_KEY } from './constants';
import { Persona } from './entity';

import type { PersonaDeletedEventData, PersonaEventData } from './types';

const toPersona = (persona: {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
  avatar_uid?: string | null;
}) =>
  new Persona(
    persona.uid,
    persona.name,
    persona.description,
    persona.creator_uid,
    persona.avatar_uid ?? null,
  );

export const usePersonasQuery = () =>
  useQuery({
    queryKey: [PERSONAS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.personas.getAll();

      if (response.error) {
        throw new Error('Failed to fetch personas');
      }

      return response.data.map(toPersona);
    },
  });

socket.on('persona:created', (persona: PersonaEventData) => {
  const newPersona = toPersona(persona);

  queryClient.setQueryData<Persona[]>([PERSONAS_QUERY_KEY], (old) =>
    old ? [...old, newPersona] : [newPersona],
  );
});

socket.on('persona:updated', (persona: PersonaEventData) => {
  const updatedPersona = toPersona(persona);

  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) =>
      old?.map((item) => (item.uid !== persona.uid ? item : updatedPersona)) ||
      [],
  );
});

socket.on('persona:deleted', (payload: PersonaDeletedEventData) => {
  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) => old?.filter((item) => item.uid !== payload.uid) || [],
  );
});
