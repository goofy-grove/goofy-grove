import type { ChatCharacter } from '@pages/chats/model';

export type ChatListItemProps = {
  uid: string;
  title: string;
  imageUrl?: string;
  avatarUid?: string | null;
  characters: Pick<
    ChatCharacter,
    'id' | 'name' | 'initials' | 'previewUrl' | 'avatarUid'
  >[];
  lastMessage?: { author: string; text: string };
  activity?: { label: string; dateTime: string };
  unreadCount?: number;
  generatingCharacterName?: string;

  onOpen: (uid: string) => void;
  onActions?: (uid: string) => void;
};
