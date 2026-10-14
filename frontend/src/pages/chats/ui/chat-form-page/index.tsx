import { IconArrowLeft } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { useCreateChatMutation } from '@pages/chats/model';

import { getApiErrorMessage } from '@shared/api';
import { Button, Form, Input, Text, useObjectUrl } from '@shared/ui';

import './styles.scss';

export const ChatFormPage = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const createMutation = useCreateChatMutation();
  const isEditMode = false;

  const [name, setName] = useState('');
  const [avatarFile, setAvatarFile] = useState<File>();
  const [errorMessage, setErrorMessage] = useState<string>();
  const avatarPreviewUrl = useObjectUrl(avatarFile);

  const handleSubmit = async () => {
    try {
      await createMutation.mutateAsync({
        name,
        avatarFile,
      });

      void navigate({ to: '/chats' });
    } catch (error) {
      console.log('Error creating chat:', error);
      setErrorMessage(getApiErrorMessage(error, t));
    }
  };

  return (
    <div className="chat-form-page">
      <div className="character-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('chat.edit_title') : t('chat.create_title')}
        </Text>

        <Button
          variant="ghost"
          disabled={createMutation.isPending}
          leftIcon={<IconArrowLeft size={18} />}
          onClick={() => void navigate({ to: '/chats' })}
        >
          {t('common.back')}
        </Button>
      </div>

      <p className="character-form-page__intro">{t('forms.chat_intro')}</p>

      <Form
        errorMessage={errorMessage}
        avatarPreviewUrl={avatarPreviewUrl}
        submitLabel={
          isEditMode ? t('forms.save_changes') : t('chat.create_title')
        }
        headingText={t('forms.chat_heading')}
        headingHint={t('forms.chat_hint')}
        hasPortrait
        onSubmit={handleSubmit}
        onPortraitChange={setAvatarFile}
        onCancel={() => void navigate({ to: '/chats' })}
      >
        <Input
          name="name"
          label={t('chat.labels.name')}
          placeholder={t('forms.chat_name')}
          disabled={createMutation.isPending}
          value={name}
          onChange={setName}
        />
      </Form>
    </div>
  );
};
