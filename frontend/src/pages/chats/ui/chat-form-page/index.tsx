import { IconArrowLeft } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCreateChatMutation,
  useChatsQuery,
  useUpdateChatMutation,
} from '@pages/chats/model';
import { ChatForm } from '@pages/chats/ui/chat-form';
import type { ChatFormValues } from '@pages/chats/ui/chat-form/types';

import { getApiErrorMessage } from '@shared/api';
import { Button, IconLoader, Text } from '@shared/ui';

import type { ChatFormPageProps } from './types';

import './styles.scss';

export const ChatFormPage: FC<ChatFormPageProps> = ({ mode, uid }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = useChatsQuery();
  const createMutation = useCreateChatMutation();
  const updateMutation = useUpdateChatMutation();
  const isEditMode = mode === 'edit';
  const mutation = isEditMode ? updateMutation : createMutation;

  const handleBack = () => void navigate({ to: '/chats' });
  const handleSubmit = async (values: ChatFormValues) => {
    if (isEditMode) {
      if (!uid) {
        return;
      }

      await updateMutation.mutateAsync({ uid, ...values });
    } else {
      await createMutation.mutateAsync(values);
    }

    handleBack();
  };
  const chat = mode === 'edit' ? data?.find((item) => item.uid === uid) : null;

  if (mode === 'edit' && isLoading) {
    return (
      <div className="chat-form-page__loader">
        <IconLoader size={64} isAnimated />
      </div>
    );
  }

  if (mode === 'edit' && !chat) {
    return (
      <div className="chat-form-page scrollbar">
        <Text>{t('chat.not_found')}</Text>

        <Button onClick={() => void navigate({ to: '/chats' })}>
          {t('common.back')}
        </Button>
      </div>
    );
  }

  return (
    <div className="chat-form-page scrollbar">
      <div className="chat-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('chat.edit_title') : t('chat.create_title')}
        </Text>

        <Button
          variant="ghost"
          disabled={mutation.isPending}
          leftIcon={<IconArrowLeft size={18} />}
          onClick={handleBack}
        >
          {t('common.back')}
        </Button>
      </div>

      <p className="chat-form-page__intro">{t('forms.chat_intro')}</p>

      <ChatForm
        key={isEditMode ? chat!.uid : 'create'}
        initialValues={{
          name: chat?.name ?? '',
        }}
        avatarUid={chat?.avatar_uid || undefined}
        isPending={mutation.isPending}
        errorMessage={
          mutation.isError ? getApiErrorMessage(mutation.error, t) : undefined
        }
        submitLabel={
          isEditMode ? t('forms.save_changes') : t('chat.create_title')
        }
        onSubmit={handleSubmit}
        onCancel={handleBack}
      />
    </div>
  );
};
