import type { ChatIdentityCharacter } from '@pages/chats/model';

export type CharacterThinkingProps = {
  character: ChatIdentityCharacter;
  onStop?: () => void;
};
