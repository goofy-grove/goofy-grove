import { usePersonasQuery } from '@entities/persona/model';
import { PersonaItem } from '@entities/persona/ui/persona-item';

import { IconLoader, Window } from '@shared/ui';

import type { PersonaListWindowProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();
  const hasPersonas = !isLoading && !!data?.length;

  return (
    <Window {...props}>
      {isLoading && (
        <div className="persona-list-loader">
          <IconLoader size={64} isAnimated />
        </div>
      )}

      <div className="persona-list scrollbar">
        {hasPersonas &&
          data.map((persona) => <PersonaItem key={persona.uid} {...persona} />)}
      </div>
    </Window>
  );
};
