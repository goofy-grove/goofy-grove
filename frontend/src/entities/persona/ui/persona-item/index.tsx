import { Avatar, Text } from '../../../../shared/ui';

import type { FC } from 'react';
import type { PersonaItemProps } from './types';

import './styles.scss';

export const PersonaItem: FC<PersonaItemProps> = ({
  uid,
  name,
  description,
}) => (
  <div className="persona-item" key={uid}>
    <Avatar />

    <div className="persona-item__info">
      <Text className="persona-item__info__name" tag="h3">
        {name}
      </Text>
      <Text className="persona-item__info__description">{description}</Text>
    </div>
  </div>
);
