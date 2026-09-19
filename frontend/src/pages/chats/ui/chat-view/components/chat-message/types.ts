import type {
  ChatMessageData,
  EditChatMessage,
  RetryChatMessage,
} from '@pages/chats/model';

export type ChatMessageProps = {
  message: ChatMessageData;
  onEdit?: EditChatMessage;
  onRetry?: RetryChatMessage;
  onEditingChange?: (id: string, editing: boolean) => void;
};
