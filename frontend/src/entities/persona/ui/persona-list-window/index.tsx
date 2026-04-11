import { IconLoader, useCurrentWindow, Window } from '../../../../shared/ui';
import { usePersonasQuery } from '../../model';
import { PersonaItem } from '../persona-item';

import type { FC } from 'react';
import type { PersonaListWindowProps } from './types';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();
  const { instanceId } = useCurrentWindow();

  return (
    <Window id={instanceId} {...props}>
      {isLoading && <IconLoader isAnimated />}

      {!isLoading &&
        data?.map((persona) => <PersonaItem key={persona.uid} {...persona} />)}
    </Window>
  );
};
