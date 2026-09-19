import {
  IconBold,
  IconItalic,
  IconUnderline,
  IconBlockquote,
} from '@tabler/icons-react';
import Placeholder from '@tiptap/extension-placeholder';
import { EditorContent, useEditor } from '@tiptap/react';
import { BubbleMenu } from '@tiptap/react/menus';
import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { richTextExtensions } from '@pages/chats/lib';

import { Button } from '@shared/ui';

import { FormattingToolbar } from './components/formatting-toolbar';

import type { SelectionMenuProps, RichEditorProps } from './types';
import type { FC } from 'react';

import '@pages/chats/ui/chat-view/components/rich-message/styles.scss';
import './styles.scss';

// BubbleMenu dispatches a TipTap transaction when these props change.
// Keep references stable so transaction-driven renders cannot feed that effect.
const selectionMenuOptions: SelectionMenuProps['options'] = {
  placement: 'top',
  offset: 12,
  strategy: 'fixed',
};

const appendSelectionMenu = () => document.body;

const showSelectionMenu: SelectionMenuProps['shouldShow'] = ({
  editor,
  from,
  to,
}) => {
  const breakpoint = getComputedStyle(document.documentElement)
    .getPropertyValue('--mobile-sm')
    .trim();

  return (
    window.matchMedia(`(max-width: ${breakpoint})`).matches &&
    editor.isEditable &&
    editor.isFocused &&
    from !== to
  );
};

export const RichEditor: FC<RichEditorProps> = ({
  initialContent,
  onSend,
  blocked = false,
  sendLabel,
  onCancel,
}) => {
  const { t, i18n } = useTranslation('translation', { keyPrefix: 'chat' });
  const [formatOpen, setFormatOpen] = useState(false);
  const extensions = useMemo(
    () => [
      ...richTextExtensions(),
      Placeholder.configure({ placeholder: () => i18n.t('chat.placeholder') }),
    ],
    [i18n],
  );
  const editor = useEditor(
    {
      extensions,
      content: initialContent,
      shouldRerenderOnTransaction: true,
      editorProps: {
        attributes: {
          class: 'chat-rich-text',
          role: 'textbox',
          'aria-label': t('editor'),
          'aria-multiline': 'true',
        },
      },
    },
    [],
  );

  useEffect(() => {
    if (!editor || editor.isDestroyed) return;
    editor.setOptions({
      editorProps: {
        attributes: {
          class: 'chat-rich-text',
          role: 'textbox',
          'aria-label': t('editor'),
          'aria-multiline': 'true',
        },
      },
    });
    editor.view.dispatch(editor.state.tr);
  }, [t, editor]);

  const send = () => {
    if (!editor || blocked || !editor.getText().trim()) return;
    onSend(editor.getJSON());
    editor.commands.clearContent();
    editor.commands.focus();
  };

  return (
    <div
      className="chat-rich-editor"
      onKeyDown={(event) => {
        if (
          event.key === 'Enter' &&
          (event.ctrlKey || event.metaKey) &&
          !event.nativeEvent.isComposing
        ) {
          event.preventDefault();
          send();
        }
      }}
    >
      {editor && (
        <>
          <div
            className={`chat-format-panel ${formatOpen ? 'chat-format-panel--open' : ''}`}
          >
            <FormattingToolbar editor={editor} />
          </div>

          <BubbleMenu
            editor={editor}
            appendTo={appendSelectionMenu}
            options={selectionMenuOptions}
            shouldShow={showSelectionMenu}
          >
            <div
              className="chat-selection-menu"
              role="group"
              aria-label={t('formatting')}
            >
              <button
                type="button"
                aria-label={t('bold')}
                aria-pressed={editor.isActive('bold')}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => editor.chain().focus().toggleBold().run()}
              >
                <IconBold size={20} />
              </button>

              <button
                type="button"
                aria-label={t('italic')}
                aria-pressed={editor.isActive('italic')}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => editor.chain().focus().toggleItalic().run()}
              >
                <IconItalic size={20} />
              </button>

              <button
                type="button"
                aria-label={t('underline')}
                aria-pressed={editor.isActive('underline')}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => editor.chain().focus().toggleUnderline().run()}
              >
                <IconUnderline size={20} />
              </button>

              <button
                type="button"
                aria-label={t('quote')}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => editor.chain().focus().toggleBlockquote().run()}
              >
                <IconBlockquote size={20} />
              </button>
            </div>
          </BubbleMenu>
        </>
      )}

      <EditorContent editor={editor} className="chat-rich-editor__input" />

      <div className="chat-rich-editor__footer">
        <button
          type="button"
          className="chat-format-toggle"
          aria-label={formatOpen ? t('closeFormat') : t('showFormat')}
          aria-expanded={formatOpen}
          onClick={() => setFormatOpen(!formatOpen)}
        >
          Aa
        </button>

        <span>{blocked ? t('draft') : t('shortcut')}</span>

        <div>
          {onCancel && (
            <Button
              size="compact"
              variant="ghost"
              type="button"
              onClick={onCancel}
            >
              {t('cancel')}
            </Button>
          )}

          <Button
            type="button"
            className="chat-send"
            disabled={!editor || !editor.getText().trim() || blocked}
            onClick={send}
          >
            {sendLabel ?? t('send')}

            <span aria-hidden="true">↗</span>
          </Button>
        </div>
      </div>
    </div>
  );
};
