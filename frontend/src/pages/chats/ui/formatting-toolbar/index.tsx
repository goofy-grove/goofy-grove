import {
  IconBold,
  IconItalic,
  IconUnderline,
  IconStrikethrough,
  IconBlockquote,
  IconList,
  IconListNumbers,
  IconH2,
  IconCode,
  IconLink,
  IconArrowBackUp,
  IconArrowForwardUp,
} from '@tabler/icons-react';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import type { FormattingToolbarProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const FormattingToolbar: FC<FormattingToolbarProps> = ({ editor }) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const [linkOpen, setLinkOpen] = useState(false);
  const [url, setUrl] = useState('');
  const [error, setError] = useState(false);
  const buttons = [
    {
      label: t('bold'),
      icon: IconBold,
      active: editor.isActive('bold'),
      run: () => editor.chain().focus().toggleBold().run(),
    },
    {
      label: t('italic'),
      icon: IconItalic,
      active: editor.isActive('italic'),
      run: () => editor.chain().focus().toggleItalic().run(),
    },
    {
      label: t('underline'),
      icon: IconUnderline,
      active: editor.isActive('underline'),
      run: () => editor.chain().focus().toggleUnderline().run(),
    },
    {
      label: t('strike'),
      icon: IconStrikethrough,
      active: editor.isActive('strike'),
      run: () => editor.chain().focus().toggleStrike().run(),
    },
    {
      label: t('heading'),
      icon: IconH2,
      active: editor.isActive('heading', { level: 2 }),
      run: () => editor.chain().focus().toggleHeading({ level: 2 }).run(),
    },
    {
      label: t('quote'),
      icon: IconBlockquote,
      active: editor.isActive('blockquote'),
      run: () => editor.chain().focus().toggleBlockquote().run(),
    },
    {
      label: t('bullet'),
      icon: IconList,
      active: editor.isActive('bulletList'),
      run: () => editor.chain().focus().toggleBulletList().run(),
    },
    {
      label: t('ordered'),
      icon: IconListNumbers,
      active: editor.isActive('orderedList'),
      run: () => editor.chain().focus().toggleOrderedList().run(),
    },
    {
      label: t('code'),
      icon: IconCode,
      active: editor.isActive('code'),
      run: () => editor.chain().focus().toggleCode().run(),
    },
  ];

  const applyLink = () => {
    try {
      const parsed = new URL(url);

      if (!['https:', 'http:'].includes(parsed.protocol))
        throw new Error('protocol');
      editor
        .chain()
        .focus()
        .extendMarkRange('link')
        .setLink({ href: parsed.href })
        .run();
      setLinkOpen(false);
      setError(false);
    } catch {
      setError(true);
    }
  };

  return (
    <>
      <div className="chat-toolbar" role="group" aria-label={t('formatting')}>
        {buttons.map(({ label, icon: Icon, active, run }) => (
          <button
            key={label}
            type="button"
            aria-label={label}
            title={label}
            aria-pressed={active}
            onClick={run}
          >
            <Icon size={18} />
          </button>
        ))}

        <button
          type="button"
          aria-label={t('link')}
          title={t('link')}
          aria-expanded={linkOpen}
          aria-pressed={editor.isActive('link')}
          onClick={() => {
            setUrl(String(editor.getAttributes('link').href ?? ''));
            setError(false);
            setLinkOpen(!linkOpen);
          }}
        >
          <IconLink size={18} />
        </button>

        <span className="chat-toolbar__separator" />

        <button
          type="button"
          aria-label={t('undo')}
          title={t('undo')}
          disabled={!editor.can().undo()}
          onClick={() => editor.chain().focus().undo().run()}
        >
          <IconArrowBackUp size={18} />
        </button>

        <button
          type="button"
          aria-label={t('redo')}
          title={t('redo')}
          disabled={!editor.can().redo()}
          onClick={() => editor.chain().focus().redo().run()}
        >
          <IconArrowForwardUp size={18} />
        </button>
      </div>

      {linkOpen && (
        <div className="chat-link-editor">
          <label>
            {t('url')}

            <input
              type="url"
              value={url}
              placeholder="https://"
              onChange={(event) => setUrl(event.target.value)}
              aria-invalid={error}
              onKeyDown={(event) => {
                if (event.key === 'Enter') {
                  event.preventDefault();
                  applyLink();
                }
                if (event.key === 'Escape') {
                  setLinkOpen(false);
                  editor.commands.focus();
                }
              }}
            />
          </label>

          <button type="button" onClick={applyLink}>
            {t('apply')}
          </button>

          <button
            type="button"
            onClick={() => {
              editor.chain().focus().unsetLink().run();
              setLinkOpen(false);
            }}
          >
            {t('unlink')}
          </button>

          <button
            type="button"
            onClick={() => {
              setLinkOpen(false);
              editor.commands.focus();
            }}
          >
            {t('cancel')}
          </button>

          {error && <p role="alert">{t('invalidUrl')}</p>}
        </div>
      )}
    </>
  );
};
