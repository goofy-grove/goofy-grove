import { IconPlusFilled } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import { usePersonasQuery } from '@entities/persona/model';
import { PERSONA_CREATE_WINDOW_KEY, PersonaItem } from '@entities/persona/ui';

import { Button, IconLoader, useWindow, Window } from '@shared/ui';

import type { PersonaListWindowProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();
  const { t } = useTranslation();
  const { openWindow } = useWindow(PERSONA_CREATE_WINDOW_KEY);

  const hasPersonas = !isLoading && !!data?.length;

  return (
    <Window {...props}>
      <div className="persona-list">
        {isLoading && (
          <div className="persona-list__loader">
            <IconLoader size={64} isAnimated />
          </div>
        )}

        <div className="persona-list__list scrollbar">
          {hasPersonas &&
            data.map((persona) => (
              <PersonaItem key={persona.uid} {...persona} />
            ))}
        </div>

        <div className="persona-list__actions">
          <Button
            className="persona-list__actions__button"
            leftIcon={<IconPlusFilled />}
            onClick={() => openWindow()}
          >
            {t('persona.window.actions.create')}
          </Button>
        </div>
      </div>
    </Window>
  );
};
