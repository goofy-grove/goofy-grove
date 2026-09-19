import { IconCheck, IconPhotoPlus } from '@tabler/icons-react';
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
  onCancel,
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];

    if (file) {
      onAvatarChange(file);
    }

    event.target.value = '';
  };

  const handleSubmit = (event: React.SubmitEvent) => {
    event.preventDefault();

    if (!isPending) {
      onSubmit();
    }
  };

  return (
    <form
      className="character-form"
      aria-busy={isPending}
      onSubmit={handleSubmit}
    >
      <div className="character-form__body">
        <section
          className="character-form__portrait"
          aria-label={t('forms.portrait')}
        >
          <span className="character-form__eyebrow">{t('forms.portrait')}</span>

          <FileAvatar
            className="character-form__avatar"
            variant="unbordered"
            fileUid={avatarUid}
            previewUrl={avatarPreviewUrl}
            alt={name || t('forms.portrait')}
          />

          <Button
            variant="ghost"
            disabled={isPending}
            leftIcon={<IconPhotoPlus size={18} />}
            onClick={() => fileInputRef.current?.click()}
          >
            {t(
              avatarUid || avatarPreviewUrl
                ? 'forms.change_portrait'
                : 'forms.add_portrait',
            )}
          </Button>

          <input
            ref={fileInputRef}
            type="file"
            accept="image/jpeg,image/png,image/gif"
            hidden
            disabled={isPending}
            onChange={handleFileChange}
          />

          <p className="character-form__hint">{t('forms.portrait_hint')}</p>
        </section>

        <div className="character-form__fields">
          <div className="character-form__section-heading">
            <h2>{t('forms.character_heading')}</h2>

            <p>{t('forms.character_hint')}</p>
          </div>

          <Input
            name="name"
            label={t('character.labels.name')}
            placeholder={t('forms.character_name')}
            disabled={isPending}
            value={name}
            onChange={onNameChange}
          />

          <Input
            name="description"
            label={t('character.labels.description')}
            placeholder={t('forms.character_description')}
            hint={t('forms.description_hint')}
            multiline
            disabled={isPending}
            value={description}
            onChange={onDescriptionChange}
          />
        </div>
      </div>

      <div className="character-form__footer">
        {errorMessage && (
          <div className="character-form__error" role="alert">
            <Alert type="error" message={errorMessage} />
          </div>
        )}

        <span className="character-form__save-note">
          {t('forms.save_hint')}
        </span>

        <div className="character-form__actions">
          <Button variant="ghost" disabled={isPending} onClick={onCancel}>
            {t('common.cancel')}
          </Button>

          <Button
            type="submit"
            disabled={isPending}
            leftIcon={
              isPending ? <IconLoader isAnimated /> : <IconCheck size={18} />
            }
          >
            {isPending ? t('forms.saving') : submitLabel}
          </Button>
        </div>
      </div>
    </form>
  );
};
