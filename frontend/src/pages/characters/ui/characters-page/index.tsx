import { IconPlusFilled } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCharactersQuery,
  useDeleteCharacterMutation,
} from '@pages/characters/model';
import { CharacterItem } from '@pages/characters/ui/character-item';

import { Button, ConfirmModal, IconLoader, Text } from '@shared/ui';

import './styles.scss';

export const CharactersPage: FC = () => {
  const { data, isLoading } = useCharactersQuery();
  const deleteCharacter = useDeleteCharacterMutation();
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [pendingDeleteUid, setPendingDeleteUid] = useState<string | null>(null);

  const hasCharacters = !isLoading && !!data?.length;

  const handleEdit = (uid: string) => {
    void navigate({ to: '/characters/$uid', params: { uid } });
  };

  const handleDelete = (uid: string) => {
    setPendingDeleteUid(uid);
  };

  const handleCancelDelete = () => {
    setPendingDeleteUid(null);
  };

  const handleConfirmDelete = () => {
    if (!pendingDeleteUid) {
      return;
    }

    void deleteCharacter
      .mutateAsync({ uid: pendingDeleteUid })
      .then(() => {
        setPendingDeleteUid(null);
      })
      .catch(() => undefined);
  };

  return (
    <div className="characters-page">
      <div className="characters-page__header">
        <Text tag="h2">{t('character.list_title')}</Text>
      </div>

      {isLoading && (
        <div className="characters-page__loader">
          <IconLoader size={64} isAnimated />
        </div>
      )}

      {!isLoading && !hasCharacters && (
        <div className="characters-page__empty">
          <Text>{t('character.empty')}</Text>
        </div>
      )}

      <div className="characters-page__list scrollbar">
        {hasCharacters &&
          data.map((character) => (
            <CharacterItem
              uid={character.uid}
              name={character.name}
              description={character.description}
              avatarUid={character.avatarUid}
              key={character.uid}
              onDelete={handleDelete}
              onEdit={handleEdit}
            />
          ))}
      </div>

      <div className="characters-page__actions">
        <Button
          className="characters-page__actions__button"
          leftIcon={<IconPlusFilled />}
          onClick={() => void navigate({ to: '/characters/new' })}
        >
          {t('character.actions.create')}
        </Button>
      </div>

      <ConfirmModal
        isOpen={pendingDeleteUid !== null}
        message={t('character.confirm_delete')}
        confirmLabel={t('character.actions.delete')}
        cancelLabel={t('common.cancel')}
        isPending={deleteCharacter.isPending}
        onConfirm={handleConfirmDelete}
        onCancel={handleCancelDelete}
      />
    </div>
  );
};
