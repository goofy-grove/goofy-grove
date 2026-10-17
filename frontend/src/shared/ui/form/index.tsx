import { IconCheck, IconPhotoPlus } from '@tabler/icons-react';
import { useRef, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Alert } from '@shared/ui/alert';
import { Button } from '@shared/ui/button';
import { FileAvatar } from '@shared/ui/file-avatar';
import { IconLoader } from '@shared/ui/icons';

import type { FormProps } from './types';

import './styles.scss';

export const Form: FC<FormProps> = ({
  errorMessage,
  isPending,
  submitLabel,
  headingHint,
  headingText,
  avatarPreviewUrl,
  hasPortrait,
  avatarUid,
  onSubmit,
  onCancel,
  onPortraitChange,
  children,
}) => {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: React.SubmitEvent) => {
    e.preventDefault();
    onSubmit?.();
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];

    if (file) {
      onPortraitChange?.(file);
    }

    event.target.value = '';
  };

  return (
    <form className="app-form" aria-busy={isPending} onSubmit={handleSubmit}>
      <div
        className={`app-form__body ${!hasPortrait ? 'app-form__body--without-portrait' : ''}`}
      >
        {hasPortrait && (
          <section
            className="app-form__portrait"
            aria-label={t('forms.portrait')}
          >
            <span className="app-form__eyebrow">{t('forms.portrait')}</span>

            <FileAvatar
              className="app-form__avatar"
              variant="unbordered"
              fileUid={avatarUid}
              previewUrl={avatarPreviewUrl}
              alt={t('forms.portrait')}
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

            <p className="app-form__hint">{t('forms.portrait_hint')}</p>
          </section>
        )}

        <div className="app-form__fields">
          <div className="app-form__section-heading">
            <h2>{headingText}</h2>

            <p>{headingHint}</p>
          </div>

          {children}
        </div>
      </div>

      <div className="app-form__footer">
        {errorMessage && (
          <div className="app-form__error" role="alert">
            <Alert type="error" message={errorMessage} />
          </div>
        )}

        <span className="app-form__save-note">{t('forms.save_hint')}</span>

        <div className="app-form__actions">
          {onCancel && (
            <Button variant="ghost" disabled={isPending} onClick={onCancel}>
              {t('common.cancel')}
            </Button>
          )}

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
