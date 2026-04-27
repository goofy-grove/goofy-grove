import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { CharacterForm } from '@entities/character/ui/character-form';

import { Window } from '@shared/ui';

import { useCharacterFormWindow } from './hooks';

import type { CharacterFormWindowProps } from './types';

import './styles.scss';

export const CharacterFormWindow: FC<CharacterFormWindowProps> = (props) => {
  const {
    mode,
    name,
    description,
    isPending,
    setName,
    setDescription,
    handleSubmit,
  } = useCharacterFormWindow(props);
  const { t } = useTranslation();

  const isEditMode = mode === 'edit';

  return (
    <Window
      {...props}
      title={
        isEditMode
          ? t('character.window.edit_title')
          : t('character.window.create_title')
      }
    >
      <CharacterForm
        name={name}
        description={description}
        isPending={isPending}
        submitLabel={
          isEditMode
            ? t('character.window.actions.edit')
            : t('character.window.actions.create')
        }
        onNameChange={setName}
        onDescriptionChange={setDescription}
        onSubmit={handleSubmit}
      />
    </Window>
  );
};
