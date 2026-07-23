import { useQuery } from '@tanstack/react-query';

import { api, socket } from '@shared/api';
import { queryClient } from '@shared/lib';

import { CHARACTERS_QUERY_KEY } from './constants';
import { Character } from './entity';

import type { CharacterDeletedEventData, CharacterEventData } from './types';

const toCharacter = (character: {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
  avatar_uid?: string | null;
}) =>
  new Character(
    character.uid,
    character.name,
    character.description,
    character.creator_uid,
    character.avatar_uid ?? null,
  );

export const useCharactersQuery = () =>
  useQuery({
    queryKey: [CHARACTERS_QUERY_KEY],
    queryFn: async () => {
      const response = await api.characters.getAll();

      if (response.error) {
        throw new Error('Failed to fetch characters');
      }

      return response.data.map(toCharacter);
    },
  });

socket.on('character:created', (character: CharacterEventData) => {
  const newCharacter = toCharacter(character);

  queryClient.setQueryData<Character[]>([CHARACTERS_QUERY_KEY], (old) =>
    old ? [...old, newCharacter] : [newCharacter],
  );
});

socket.on('character:updated', (character: CharacterEventData) => {
  const updatedCharacter = toCharacter(character);

  queryClient.setQueryData<Character[]>(
    [CHARACTERS_QUERY_KEY],
    (old) =>
      old?.map((item) =>
        item.uid !== character.uid ? item : updatedCharacter,
      ) || [],
  );
});

socket.on('character:deleted', (payload: CharacterDeletedEventData) => {
  queryClient.setQueryData<Character[]>(
    [CHARACTERS_QUERY_KEY],
    (old) => old?.filter((item) => item.uid !== payload.uid) || [],
  );
});
