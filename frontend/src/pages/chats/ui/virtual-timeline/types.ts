import type {
  ChatMessageData,
  EditChatMessage,
  RetryChatMessage,
  LoadChatHistory,
} from '@pages/chats/model';

import type { ReactNode } from 'react';

export type VirtualTimelineProps = {
  messages: ChatMessageData[];
  hasOlder?: boolean;
  loadOlder?: LoadChatHistory;
  pageSize?: number;
  onEdit?: EditChatMessage;
  onRetry?: RetryChatMessage;
  footer?: ReactNode;
  empty?: ReactNode;
};
