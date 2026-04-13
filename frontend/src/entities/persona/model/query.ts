import { useQuery } from '@tanstack/react-query';

import { api } from '@shared/api';

import { PERSONAS_QUERY_KEY } from './constants';
import { Persona } from './entity';

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
