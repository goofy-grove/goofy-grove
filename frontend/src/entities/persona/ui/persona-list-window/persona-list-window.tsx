import { IconPlusFilled } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import { usePersonasQuery } from '@entities/persona/model';
import { PersonaItem, PERSONA_FORM_WINDOW_KEY } from '@entities/persona/ui';
import type { PersonaFormWindowData } from '@entities/persona/ui';

import { Button, IconLoader, useWindow, Window } from '@shared/ui';

import type { PersonaListWindowProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaListWindow: FC<PersonaListWindowProps> = (props) => {
  const { data, isLoading } = usePersonasQuery();
  const { t } = useTranslation();
  const { openWindow } = useWindow<PersonaFormWindowData>(
    PERSONA_FORM_WINDOW_KEY,
  );

  const hasPersonas = !isLoading && !!data?.length;
  const handleEdit = (uid: string) => {
    const persona = data?.find((item) => item.uid === uid);

    if (!persona) {
      return;
    }

    openWindow({
      mode: 'edit',
      uid: persona.uid,
      initialName: persona.name,
      initialDescription: persona.description,
    });
  };

  return (
    <Window {...props} title={t('persona.window.list_title')}>
      <div className="persona-list">
        {isLoading && (
          <div className="persona-list__loader">
            <IconLoader size={64} isAnimated />
          </div>
        )}

        <div className="persona-list__list scrollbar">
          {hasPersonas &&
            data.map((persona) => (
              <PersonaItem {...persona} key={persona.uid} onEdit={handleEdit} />
            ))}
        </div>

        <div className="persona-list__actions">
          <Button
            className="persona-list__actions__button"
            leftIcon={<IconPlusFilled />}
            onClick={() => openWindow({ mode: 'create' })}
          >
            {t('persona.window.actions.create')}
          </Button>
        </div>
      </div>
    </Window>
  );
};
