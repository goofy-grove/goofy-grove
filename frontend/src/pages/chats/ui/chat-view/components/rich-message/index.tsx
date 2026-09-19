import { EditorContent, useEditor } from '@tiptap/react';
import { useEffect, useMemo } from 'react';

import { richTextExtensions } from '@pages/chats/lib/rich-text-extensions';

import type { RichMessageProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const RichMessage: FC<RichMessageProps> = ({ content }) => {
  const extensions = useMemo(() => richTextExtensions(true), []);
  const editor = useEditor({
    extensions,
    content,
    editable: false,
    editorProps: { attributes: { class: 'chat-rich-text', tabindex: '-1' } },
  });

  useEffect(() => {
    if (
      editor &&
      !editor.isDestroyed &&
      !editor.state.doc.eq(editor.schema.nodeFromJSON(content))
    ) {
      editor.commands.setContent(content, { emitUpdate: false });
    }
  }, [editor, content]);

  return <EditorContent editor={editor} className="chat-message__document" />;
};

