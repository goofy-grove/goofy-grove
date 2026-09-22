import { IconPlusFilled } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCharacterActions,
  useCharactersQuery,
} from '@pages/characters/model';
import { CharacterItem } from '@pages/characters/ui/character-item';

import {
  ActionSheet,
  Button,
  ConfirmModal,
  GroveScene,
  IconLoader,
  PageHeader,
  Text,
  useBreakpoints,
} from '@shared/ui';

import './styles.scss';

export const CharactersPage: FC = () => {
  const {
    actions,
    pendingDeleteUid,
    isActionSheetOpen,
    handleOpenActionSheet,
    handleCloseActionSheet,
    deleteCharacter,
    handleDelete,
    handleCancelDelete,
    handleConfirmDelete,
    handleEdit,
  } = useCharacterActions();
  const { isMobileSm } = useBreakpoints();
  const { data, isLoading } = useCharactersQuery();

  const { t } = useTranslation();
  const navigate = useNavigate();

  const hasCharacters = !isLoading && !!data?.length;

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
              showActions={!isMobileSm}
              onLongPress={handleOpenActionSheet}
              onClick={handleEdit}
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

      <ActionSheet
        items={actions}
        isOpened={isActionSheetOpen}
        onClose={handleCloseActionSheet}
        showClose
      />
    </div>
  );
};
