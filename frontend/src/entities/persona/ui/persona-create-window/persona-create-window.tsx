import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button, IconLoader, Input, Window } from '@shared/ui';

import { usePersonaCreateWindow } from './hooks';

import type { PersonaCreateWindowProps } from './types';

import './styles.scss';

export const PersonaCreateWindow: FC<PersonaCreateWindowProps> = (props) => {
  const {
    name,
    description,
    isPending,
    setName,
    setDescription,
    handleCreate,
  } = usePersonaCreateWindow();
  const { t } = useTranslation();

  return (
    <Window {...props} title={t('person.window.create_title')}>
      <div className="persona-create-window">
        <div className="persona-create-window__content scrollbar">
          <Input
            placeholder="Name"
            label="Name"
            disabled={isPending}
            value={name}
            onChange={setName}
          />

          <Input
            placeholder="Description"
            label="Description"
            multiline
            disabled={isPending}
            defaultValue={description}
            onChange={setDescription}
          />
        </div>

        <Button
          className="persona-create-window__button"
          onClick={handleCreate}
          disabled={isPending}
          leftIcon={isPending && <IconLoader isAnimated />}
        >
          Create
        </Button>
      </div>
    </Window>
  );
};
