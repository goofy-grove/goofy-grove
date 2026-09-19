import type { ChatPersona } from '@pages/chats/model';

export type PersonaSelectProps = {
  personas: ChatPersona[];
  selectedId: string;
  onChange: (id: string) => void;
};
