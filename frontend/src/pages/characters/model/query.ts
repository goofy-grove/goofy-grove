import { useQuery } from '@tanstack/react-query';

import { api, socket } from '@shared/api';
import type { Character } from '@shared/api';
import { queryClient } from '@shared/lib';

import { CHARACTERS_QUERY_KEY } from './constants';

import type { CharacterDeletedEventData } from './types';

export const useCharactersQuery = () =>
  useQuery({
    queryKey: [CHARACTERS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.characters.getAll();

      if (response.error) {
        throw new Error('Failed to fetch characters');
      }

      return response.data;
    },
  });

socket.on('character:created', (character: Character) => {
  queryClient.setQueryData<Character[]>([CHARACTERS_QUERY_KEY], (old) =>
    old ? [...old, character] : [character],
  );
});

socket.on('character:updated', (character: Character) => {
  queryClient.setQueryData<Character[]>(
    [CHARACTERS_QUERY_KEY],
    (old) =>
      old?.map((item) => (item.uid !== character.uid ? item : character)) || [],
  );
});

socket.on('character:deleted', (payload: CharacterDeletedEventData) => {
  queryClient.setQueryData<Character[]>(
    [CHARACTERS_QUERY_KEY],
    (old) => old?.filter((item) => item.uid !== payload.uid) || [],
  );
});
