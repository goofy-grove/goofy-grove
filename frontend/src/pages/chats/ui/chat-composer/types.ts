import type { ChatPersona } from '@pages/chats/model';

import type { JSONContent } from '@tiptap/react';

export type ChatComposerProps = {
  personas: ChatPersona[];
  selectedId: string;
  blocked?: boolean;
  initialContent?: JSONContent;

  onPersonaChange: (id: string) => void;
  onSend: (content: JSONContent) => void;
};
