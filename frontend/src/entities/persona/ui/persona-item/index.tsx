import { IconDotsVerticalFilled } from '@tabler/icons-react';

import { Avatar, Button, Dropdown, Text } from '../../../../shared/ui';

import type { FC } from 'react';
import type { PersonaItemProps } from './types';

import './styles.scss';

export const PersonaItem: FC<PersonaItemProps> = ({
  uid,
  name,
  description,
}) => (
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
        trigger={
          <Button variant="ghost" leftIcon={<IconDotsVerticalFilled />} />
        }
      >
        <div>Actions</div>
      </Dropdown>
    </div>
  </div>
);
