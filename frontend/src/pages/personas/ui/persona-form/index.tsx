import { type ChangeEvent, type FC, useRef } from 'react';
import { useTranslation } from 'react-i18next';

import { Alert, Button, FileAvatar, IconLoader, Input } from '@shared/ui';

import type { PersonaFormProps } from './types';

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
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0] ?? null;
    onAvatarChange(file);
  };

  return (
    <div className="persona-form">
      <div className="persona-form__content scrollbar">
        {errorMessage && <Alert type="error" message={errorMessage} closable />}

        <div className="persona-form__avatar">
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
            {t('persona.labels.avatar')}
          </Button>
        </div>

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
        className="persona-form__button"
        onClick={onSubmit}
        disabled={isPending}
        leftIcon={isPending && <IconLoader isAnimated />}
      >
        {submitLabel}
      </Button>
    </div>
  );
};
