import { useQuery } from '@tanstack/react-query';

import { api } from '@shared/api';

import { Persona } from './entity';

export const usePersonasQuery = () =>
  useQuery({
    queryKey: ['personas'],
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
