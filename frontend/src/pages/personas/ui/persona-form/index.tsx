import { useTranslation } from 'react-i18next';

import { Input } from '@shared/ui';
import { Form } from '@shared/ui/form';

import type { PersonaFormProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaForm: FC<PersonaFormProps> = ({
  name,
  description,
  isPending,
  submitLabel,
  errorMessage,
  avatarUid,
  avatarPreviewUrl,
  onNameChange,
  onDescriptionChange,
  onAvatarChange,
  onSubmit,
  onCancel,
}) => {
  const { t } = useTranslation();

  return (
    <Form
      errorMessage={errorMessage}
      avatarUid={avatarUid}
      avatarPreviewUrl={avatarPreviewUrl}
      submitLabel={submitLabel}
      headingText={t('forms.persona_heading')}
      headingHint={t('forms.persona_hint')}
      hasPortrait
      onSubmit={onSubmit}
      onCancel={onCancel}
      onPortraitChange={onAvatarChange}
    >
      <Input
        name="name"
        label={t('persona.labels.name')}
        placeholder={t('forms.persona_name')}
        disabled={isPending}
        value={name}
        onChange={onNameChange}
      />

      <Input
        className="persona-form__textarea"
        name="description"
        label={t('persona.labels.description')}
        placeholder={t('forms.persona_description')}
        hint={t('forms.description_hint')}
        multiline
        disabled={isPending}
        value={description}
        onChange={onDescriptionChange}
      />
    </Form>
  );
};
