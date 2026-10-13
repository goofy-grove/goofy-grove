import { useTranslation } from 'react-i18next';

import { Input } from '@shared/ui';
import { Form } from '@shared/ui/form';
import './styles.scss';

import type { CharacterFormProps } from './types';
import type { FC } from 'react';

export const CharacterForm: FC<CharacterFormProps> = ({
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
      headingText={t('forms.character_heading')}
      headingHint={t('forms.character_hint')}
      hasPortrait
      onSubmit={onSubmit}
      onCancel={onCancel}
      onPortraitChange={onAvatarChange}
    >
      <Input
        name="name"
        label={t('character.labels.name')}
        placeholder={t('forms.character_name')}
        disabled={isPending}
        value={name}
        onChange={onNameChange}
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
        onChange={onDescriptionChange}
      />
    </Form>
  );
};
