import { IconDotsVertical, IconPencil, IconTrash } from '@tabler/icons-react';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button, FileAvatar, Text } from '@shared/ui';

import type { CharacterItemProps } from './types';

import './styles.scss';

export const CharacterItem: FC<CharacterItemProps> = ({
  uid,
  name,
  description,
  avatarUid,
  showActions,

  onEdit,
  onDelete,
  onClick,
  onOpenAction,
}) => {
  const { t } = useTranslation();

  const handleOpenAction = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();

    onOpenAction?.(uid);
  };

  return (
    <article onClick={() => onClick?.(uid)} className="character-item">
      <FileAvatar
        className="character-item__avatar"
        variant="unbordered"
        fileUid={avatarUid}
        alt={name}
      />

      <div className="character-item__info">
        <Text className="character-item__info__name" tag="h3">
          {name}
        </Text>

        <Text className="character-item__info__description">{description}</Text>
      </div>

      {showActions && (
        <div className="character-item__actions">
          <Button
            variant="ghost"
            leftIcon={<IconPencil size={17} />}
            onClick={() => onEdit?.(uid)}
          >
            {t('character.actions.edit')}
          </Button>

          <Button
            variant="ghost"
            color="error"
            leftIcon={<IconTrash size={17} />}
            onClick={() => onDelete?.(uid)}
          >
            {t('character.actions.delete')}
          </Button>
        </div>
      )}

      {!showActions && (
        <Button
          className="character-item__dots-actions"
          variant="ghost"
          leftIcon={<IconDotsVertical size={17} />}
          onClick={handleOpenAction}
        />
      )}
    </article>
  );
};
