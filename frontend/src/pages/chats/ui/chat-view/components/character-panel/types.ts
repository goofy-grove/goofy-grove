import type { ChatCharacter } from '@pages/chats/model';

export type CharacterPanelProps = {
  characters: ChatCharacter[];
  currentUserId: string;
  generatingId?: string | null;
  disabled?: boolean;
  onTrigger: (id: string) => void;
  onEdit: (id: string) => void;
};
