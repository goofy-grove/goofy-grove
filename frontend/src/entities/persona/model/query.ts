import { useQuery } from '@tanstack/react-query';

import { api, socket } from '@shared/api';
import { queryClient } from '@shared/lib';

import { PERSONAS_QUERY_KEY } from './constants';
import { Persona } from './entity';

import type { PersonaDeletedEventData, PersonaEventData } from './types';

export const usePersonasQuery = () =>
  useQuery({
    queryKey: [PERSONAS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.personas.getAll();

      if (response.error) {
        throw new Error('Failed to fetch personas');
      }

      return response.data.map(
        (persona) =>
          new Persona(
            persona.uid,
            persona.name,
            persona.description,
            persona.creator_uid,
          ),
      );
    },
  });

socket.on('persona:created', (persona: PersonaEventData) => {
  const newPersona = new Persona(
    persona.id,
    persona.name,
    persona.description,
    persona.creator_uid,
  );

  queryClient.setQueryData<Persona[]>([PERSONAS_QUERY_KEY], (old) =>
    old ? [...old, newPersona] : [newPersona],
  );
});

socket.on('persona:updated', (persona: PersonaEventData) => {
  const updatedPersona = new Persona(
    persona.id,
    persona.name,
    persona.description,
    persona.creator_uid,
  );

  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) =>
      old?.map((item) => (item.uid !== persona.id ? item : updatedPersona)) ||
      [],
  );
});

socket.on('persona:deleted', (payload: PersonaDeletedEventData) => {
  queryClient.setQueryData<Persona[]>(
    [PERSONAS_QUERY_KEY],
    (old) => old?.filter((item) => item.uid !== payload.id) || [],
  );
});
