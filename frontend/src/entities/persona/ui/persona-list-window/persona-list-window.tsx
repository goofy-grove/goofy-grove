import { IconLoader, Window } from '../../../../shared/ui';
import { usePersonasQuery } from '../../model';
import { PersonaItem } from '../persona-item';

import type { FC } from 'react';
import type { PersonaListWindowProps } from './types';

import './styles.scss';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();

  return (
    <Window {...props}>
      {isLoading && <IconLoader isAnimated />}

      <div className="persona-list scrollbar">
        {!isLoading &&
          data?.map((persona) => (
            <PersonaItem key={persona.uid} {...persona} />
          ))}
      </div>
    </Window>
  );
};
