import { type ChangeEvent, type FC, useRef } from 'react';
import { useTranslation } from 'react-i18next';

import { Alert, Button, FileAvatar, IconLoader, Input } from '@shared/ui';

import type { CharacterFormProps } from './types';

import './styles.scss';

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
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0] ?? null;

    onAvatarChange(file);
  };

  return (
    <div className="character-form">
      <div className="character-form__content scrollbar">
        {errorMessage && <Alert type="error" message={errorMessage} closable />}

        <div className="character-form__avatar">
          <FileAvatar
            size="large"
            fileUid={avatarUid}
            previewUrl={avatarPreviewUrl}
          />

          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            hidden
            disabled={isPending}
            onChange={handleFileChange}
          />

          <Button
            variant="ghost"
            disabled={isPending}
            onClick={() => fileInputRef.current?.click()}
          >
            {t('character.labels.avatar')}
          </Button>
        </div>

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
        className="character-form__button"
        onClick={onSubmit}
        disabled={isPending}
        leftIcon={isPending && <IconLoader isAnimated />}
      >
        {submitLabel}
      </Button>
    </div>
  );
};
