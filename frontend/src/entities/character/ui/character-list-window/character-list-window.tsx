import { IconPlusFilled } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import {
  useCharactersQuery,
  useDeleteCharacterMutation,
} from '@entities/character/model';
import {
  CharacterItem,
  CHARACTER_FORM_WINDOW_KEY,
} from '@entities/character/ui';
import type { CharacterFormWindowData } from '@entities/character/ui';

import { Button, IconLoader, useWindow, Window } from '@shared/ui';

import type { CharacterListWindowProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const CharacterListWindow: FC<CharacterListWindowProps> = (props) => {
  const { data, isLoading } = useCharactersQuery();
  const deleteCharacter = useDeleteCharacterMutation();
  const { t } = useTranslation();
  const { openWindow } = useWindow<CharacterFormWindowData>(
    CHARACTER_FORM_WINDOW_KEY,
  );

  const hasCharacters = !isLoading && !!data?.length;
  const handleEdit = (uid: string) => {
    const character = data?.find((item) => item.uid === uid);

    if (!character) {
      return;
    }

    openWindow({
      mode: 'edit',
      uid: character.uid,
      initialName: character.name,
      initialDescription: character.description,
    });
  };

  const handleDelete = (uid: string) => {
    void deleteCharacter.mutateAsync({ uid }).catch(() => undefined);
  };

  return (
    <Window {...props} title={t('character.window.list_title')}>
      <div className="character-list">
        {isLoading && (
          <div className="character-list__loader">
            <IconLoader size={64} isAnimated />
          </div>
        )}

        <div className="character-list__list scrollbar">
          {hasCharacters &&
            data.map((character) => (
              <CharacterItem
                {...character}
                key={character.uid}
                onDelete={handleDelete}
                onEdit={handleEdit}
              />
            ))}
        </div>

        <div className="character-list__actions">
          <Button
            className="character-list__actions__button"
            leftIcon={<IconPlusFilled />}
            onClick={() => openWindow({ mode: 'create' })}
          >
            {t('character.window.actions.create')}
          </Button>
        </div>
      </div>
    </Window>
  );
};
