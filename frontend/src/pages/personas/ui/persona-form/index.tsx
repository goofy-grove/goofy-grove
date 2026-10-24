import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { Input, useObjectUrl } from '@shared/ui';
import { Form } from '@shared/ui/form';

import type { PersonaFormProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaForm: FC<PersonaFormProps> = ({
  initialValues,
  isPending,
  submitLabel,
  errorMessage,
  avatarUid,
  onSubmit,
  onCancel,
}) => {
  const { t } = useTranslation();
  const [name, setName] = useState(initialValues.name);
  const [description, setDescription] = useState(initialValues.description);
  const [avatarFile, setAvatarFile] = useState<File>();
  const avatarPreviewUrl = useObjectUrl(avatarFile);

  const handleSubmit = () => {
    if (!isPending) {
      onSubmit({ name, description, avatarFile });
    }
  };

  return (
    <Form
      isPending={isPending}
      errorMessage={errorMessage}
      avatarUid={avatarUid}
      avatarPreviewUrl={avatarPreviewUrl}
      submitLabel={submitLabel}
      headingText={t('forms.persona_heading')}
      headingHint={t('forms.persona_hint')}
      hasPortrait
      onSubmit={handleSubmit}
      onCancel={onCancel}
      onPortraitChange={setAvatarFile}
    >
      <Input
        name="name"
        label={t('persona.labels.name')}
        placeholder={t('forms.persona_name')}
        disabled={isPending}
        value={name}
        onChange={setName}
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
        onChange={setDescription}
      />
    </Form>
  );
};
