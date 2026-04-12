import { IconLoader, Window } from '../../../../shared/ui';
import { usePersonasQuery } from '../../model';
import { PersonaItem } from '../persona-item';

import type { FC } from 'react';
import type { PersonaListWindowProps } from './types';

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
