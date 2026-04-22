import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button, IconLoader, Input } from '@shared/ui';

import type { PersonaFormProps } from './types';

export const PersonaForm: FC<PersonaFormProps> = ({
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
    <div className="persona-form-window">
      <div className="persona-form-window__content scrollbar">
        <Input
          placeholder={t('persona.labels.name')}
          label={t('persona.labels.name')}
          disabled={isPending}
          value={name}
          onChange={onNameChange}
        />

        <Input
          placeholder={t('persona.labels.description')}
          label={t('persona.labels.description')}
          multiline
          disabled={isPending}
          value={description}
          onChange={onDescriptionChange}
        />
      </div>

      <Button
        className="persona-form-window__button"
        onClick={onSubmit}
        disabled={isPending}
        leftIcon={isPending && <IconLoader isAnimated />}
      >
        {submitLabel}
      </Button>
    </div>
  );
};
