import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button, IconLoader, Input } from '@shared/ui';

import type { CharacterFormProps } from './types';

export const CharacterForm: FC<CharacterFormProps> = ({
  name,
  description,
  isPending,
  submitLabel,
  onNameChange,
  onDescriptionChange,
  onSubmit,
}) => {
  const { t } = useTranslation();

  return (
    <div className="character-form-window">
      <div className="character-form-window__content scrollbar">
        <Input
          placeholder={t('character.labels.name')}
          label={t('character.labels.name')}
          disabled={isPending}
          value={name}
          onChange={onNameChange}
        />

        <Input
          placeholder={t('character.labels.description')}
          label={t('character.labels.description')}
          multiline
          disabled={isPending}
          value={description}
          onChange={onDescriptionChange}
        />
      </div>

      <Button
        className="character-form-window__button"
        onClick={onSubmit}
        disabled={isPending}
        leftIcon={isPending && <IconLoader isAnimated />}
      >
        {submitLabel}
      </Button>
    </div>
  );
};
