import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { PersonaForm } from '@entities/persona/ui/persona-form';

import { Window } from '@shared/ui';

import { usePersonaFormWindow } from './hooks';

import type { PersonaFormWindowProps } from './types';

import './styles.scss';

export const PersonaFormWindow: FC<PersonaFormWindowProps> = (props) => {
  const {
    mode,
    name,
    description,
    isPending,
    setName,
    setDescription,
    handleSubmit,
  } = usePersonaFormWindow(props);
  const { t } = useTranslation();

  const isEditMode = mode === 'edit';

  return (
    <Window
      {...props}
      title={
        isEditMode
          ? t('persona.window.edit_title')
          : t('persona.window.create_title')
      }
    >
      <PersonaForm
        name={name}
        description={description}
        isPending={isPending}
        submitLabel={
          isEditMode
            ? t('persona.window.actions.edit')
            : t('persona.window.actions.create')
        }
        onNameChange={setName}
        onDescriptionChange={setDescription}
        onSubmit={handleSubmit}
      />
    </Window>
  );
};
