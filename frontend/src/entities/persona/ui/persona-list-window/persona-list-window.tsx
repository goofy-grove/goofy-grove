import { IconLoader, Window } from '../../../../shared/ui';
import { usePersonasQuery } from '../../model';
import { PersonaItem } from '../persona-item';

import { PERSONA_LIST_WINDOW_KEY } from './constants';

import type { FC } from 'react';
import type { PersonaListWindowProps } from './types';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();

  return (
    <Window id={PERSONA_LIST_WINDOW_KEY} {...props}>
      {isLoading && <IconLoader isAnimated />}

      {!isLoading &&
        data?.map((persona) => <PersonaItem key={persona.uid} {...persona} />)}
    </Window>
  );
};
