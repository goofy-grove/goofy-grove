import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { Input, useObjectUrl } from '@shared/ui';
import { Form } from '@shared/ui/form';

import type { ChatFormProps } from './types';
import type { FC } from 'react';

export const ChatForm: FC<ChatFormProps> = ({
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
  const [avatarFile, setAvatarFile] = useState<File>();
  const avatarPreviewUrl = useObjectUrl(avatarFile);

  const handleSubmit = () => {
    if (!isPending) {
      onSubmit({ name, avatarFile });
    }
  };

  return (
    <Form
      isPending={isPending}
      errorMessage={errorMessage}
      avatarUid={avatarUid}
      avatarPreviewUrl={avatarPreviewUrl}
      submitLabel={submitLabel}
      headingText={t('forms.chat_heading')}
      headingHint={t('forms.chat_hint')}
      hasPortrait
      onSubmit={handleSubmit}
      onCancel={onCancel}
      onPortraitChange={setAvatarFile}
    >
      <Input
        name="name"
        label={t('chat.labels.name')}
        placeholder={t('forms.chat_name')}
        disabled={isPending}
        value={name}
        onChange={setName}
      />
    </Form>
  );
};
