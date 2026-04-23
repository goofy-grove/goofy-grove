import {
  IconDotsVerticalFilled,
  IconPencil,
  IconTrash,
} from '@tabler/icons-react';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Avatar, Button, Dropdown, Text } from '@shared/ui';

import type { PersonaItemProps } from './types';

import './styles.scss';

export const PersonaItem: FC<PersonaItemProps> = ({
  uid,
  name,
  description,
  onEdit,
  onDelete,
}) => {
  const { t } = useTranslation();
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  const handleDropdownShow = () => {
    setIsDropdownOpen(true);
  };

  const handleEdit = () => {
    setIsDropdownOpen(false);
    onEdit?.(uid);
  };

  const handleDelete = () => {
    setIsDropdownOpen(false);
    onDelete?.(uid);
  };

  return (
    <div className="persona-item" key={uid}>
      <Avatar className="persona-item__avatar" />

      <div className="persona-item__info">
        <Text className="persona-item__info__name" tag="h3">
          {name}
        </Text>
        <Text className="persona-item__info__description">{description}</Text>
      </div>

      <div className="persona-item__actions">
        <Dropdown
          isOpen={isDropdownOpen}
          onShow={handleDropdownShow}
          trigger={
            <Button variant="ghost" leftIcon={<IconDotsVerticalFilled />} />
          }
        >
          <div className="persona-item__actions__menu">
            <Button
              variant="ghost"
              leftIcon={<IconPencil size={18} />}
              onClick={handleEdit}
            >
              {t('persona.window.actions.edit')}
            </Button>
            <Button
              variant="ghost"
              color="error"
              leftIcon={<IconTrash size={18} />}
              onClick={handleDelete}
            >
              {t('persona.window.actions.delete')}
            </Button>
          </div>
        </Dropdown>
      </div>
    </div>
  );
};
