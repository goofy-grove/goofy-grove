import { IconPencil, IconTrash } from '@tabler/icons-react';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button, FileAvatar, Text } from '@shared/ui';

import type { PersonaItemProps } from './types';

import './styles.scss';

export const PersonaItem: FC<PersonaItemProps> = ({
  uid,
  name,
  description,
  avatarUid,
  onEdit,
  onDelete,
}) => {
  const { t } = useTranslation();

  return (
    <article className="persona-item">
      <FileAvatar
        className="persona-item__avatar"
        variant="unbordered"
        fileUid={avatarUid}
        alt={name}
      />

      <div className="persona-item__info">
        <Text className="persona-item__info__name" tag="h3">
          {name}
        </Text>

        <Text className="persona-item__info__description">{description}</Text>
      </div>

      <div className="persona-item__actions">
        <Button
          variant="ghost"
          leftIcon={<IconPencil size={17} />}
          onClick={() => onEdit?.(uid)}
        >
          {t('persona.actions.edit')}
        </Button>

        <Button
          variant="ghost"
          color="error"
          leftIcon={<IconTrash size={17} />}
          onClick={() => onDelete?.(uid)}
        >
          {t('persona.actions.delete')}
        </Button>
      </div>
    </article>
  );
};
