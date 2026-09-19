import type {
  ChatPersona,
  ChatMessageData,
  ChatCharacter,
  ChatUser,
  EditChatMessage,
  RetryChatMessage,
  LoadChatHistory,
} from '@pages/chats/model';

import type { JSONContent } from '@tiptap/react';

export type ChatViewProps = {
  title: string;
  subtitle?: string;
  users: ChatUser[];
  currentUserId: string;
  characters: ChatCharacter[];
  myPersonas: ChatPersona[];
  selectedPersonaId: string;
  messages: ChatMessageData[];
  onPersonaChange: (id: string) => void;
  onSend: (content: JSONContent) => void;
  onTriggerCharacter: (id: string) => void;
  onEditCharacter: (id: string) => void;
  onStop?: () => void;
  onEditMessage?: EditChatMessage;
  onRetry?: RetryChatMessage;
  loadOlder?: LoadChatHistory;
  hasOlder?: boolean;
  pageSize?: number;
  generatingCharacterId?: string | null;
  readOnly?: boolean;
  notice?: string;
  backgroundUrl?: string;
};
