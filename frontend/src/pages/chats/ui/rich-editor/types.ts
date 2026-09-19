import type { JSONContent } from '@tiptap/react';
import type { BubbleMenu } from '@tiptap/react/menus';
import type { ComponentProps } from 'react';

export type RichEditorProps = {
  initialContent?: JSONContent;
  onSend: (content: JSONContent) => void;
  blocked?: boolean;
  sendLabel?: string;
  onCancel?: () => void;
};

export type SelectionMenuProps = ComponentProps<typeof BubbleMenu>;
