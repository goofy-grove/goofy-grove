import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { Input, useObjectUrl } from '@shared/ui';
import { Form } from '@shared/ui/form';

import type { CharacterFormProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const CharacterForm: FC<CharacterFormProps> = ({
  initialValues,
  isPending,
  submitLabel,
  errorMessage,
  avatarUid,
  avatarPreviewUrl,
  onSubmit,
  onCancel,
}) => {
  const { t } = useTranslation();
  const [name, setName] = useState(initialValues.name);
  const [description, setDescription] = useState(initialValues.description);
  const [avatarFile, setAvatarFile] = useState<File>();
  const selectedAvatarPreviewUrl = useObjectUrl(avatarFile);

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
      avatarPreviewUrl={selectedAvatarPreviewUrl ?? avatarPreviewUrl}
      submitLabel={submitLabel}
      headingText={t('forms.character_heading')}
      headingHint={t('forms.character_hint')}
      hasPortrait
      onSubmit={handleSubmit}
      onCancel={onCancel}
      onPortraitChange={setAvatarFile}
    >
      <Input
        name="name"
        label={t('character.labels.name')}
        placeholder={t('forms.character_name')}
        disabled={isPending}
        value={name}
        onChange={setName}
      />

      <Input
        className="character-form__textarea"
        name="description"
        label={t('character.labels.description')}
        placeholder={t('forms.character_description')}
        hint={t('forms.description_hint')}
        multiline
        disabled={isPending}
        value={description}
        onChange={setDescription}
      />
    </Form>
  );
};
