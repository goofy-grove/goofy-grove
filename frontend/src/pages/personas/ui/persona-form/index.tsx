import { IconCheck, IconPhotoPlus } from '@tabler/icons-react';
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
  onCancel,
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];

    if (file) onAvatarChange(file);
    event.target.value = '';
  };

  return (
    <form
      className="persona-form"
      aria-busy={isPending}
      onSubmit={(event) => {
        event.preventDefault();
        if (!isPending) onSubmit();
      }}
    >
      <div className="persona-form__body">
        <section
          className="persona-form__portrait"
          aria-label={t('forms.portrait')}
        >
          <span className="persona-form__eyebrow">{t('forms.portrait')}</span>

          <FileAvatar
            className="persona-form__avatar"
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

          <p className="persona-form__hint">{t('forms.portrait_hint')}</p>
        </section>

        <div className="persona-form__fields">
          <div className="persona-form__section-heading">
            <h2>{t('forms.persona_heading')}</h2>

            <p>{t('forms.persona_hint')}</p>
          </div>

          <Input
            name="name"
            label={t('persona.labels.name')}
            placeholder={t('forms.persona_name')}
            disabled={isPending}
            value={name}
            onChange={onNameChange}
          />

          <Input
            name="description"
            label={t('persona.labels.description')}
            placeholder={t('forms.persona_description')}
            hint={t('forms.description_hint')}
            multiline
            disabled={isPending}
            value={description}
            onChange={onDescriptionChange}
          />
        </div>
      </div>

      <div className="persona-form__footer">
        {errorMessage && (
          <div className="persona-form__error" role="alert">
            <Alert type="error" message={errorMessage} />
          </div>
        )}

        <span className="persona-form__save-note">{t('forms.save_hint')}</span>

        <div className="persona-form__actions">
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
