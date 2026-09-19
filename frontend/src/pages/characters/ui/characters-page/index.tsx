import { IconPlusFilled } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCharactersQuery,
  useDeleteCharacterMutation,
} from '@pages/characters/model';
import { CharacterItem } from '@pages/characters/ui/character-item';

import {
  Button,
  ConfirmModal,
  GroveScene,
  IconLoader,
  PageHeader,
  Text,
} from '@shared/ui';

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
      <PageHeader title={t('character.list_title')}>
        <Button
          leftIcon={<IconPlusFilled size={18} />}
          onClick={() => void navigate({ to: '/characters/new' })}
        >
          {t('character.create_title')}
        </Button>
      </PageHeader>

      <p className="characters-page__intro">{t('grove.characters_intro')}</p>

      {isLoading && (
        <div className="characters-page__loader">
          <IconLoader size={64} isAnimated />
        </div>
      )}

      {!isLoading && !hasCharacters && (
        <div className="characters-page__empty">
          <GroveScene />

          <Text tag="h3">{t('character.empty')}</Text>

          <Text>{t('grove.characters_empty')}</Text>
        </div>
      )}

      <div className="characters-page__list scrollbar">
        {hasCharacters &&
          data.map((character) => (
            <CharacterItem
              uid={character.uid}
              name={character.name}
              description={character.description}
              avatarUid={character.avatar_uid}
              key={character.uid}
              onDelete={handleDelete}
              onEdit={handleEdit}
            />
          ))}
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
