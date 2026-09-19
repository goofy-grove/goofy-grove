import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { expect, userEvent, within } from 'storybook/test';

import { ChatComposer } from '@pages/chats/ui/chat-view/components/chat-composer';
import { RichMessage } from '@pages/chats/ui/chat-view/components/rich-message';

import { createScene } from './model';

import type { Meta, StoryObj } from '@storybook/react-vite';
import type { JSONContent } from '@tiptap/react';

function ComposerPreview({
  blocked = false,
  rich = false,
}: {
  blocked?: boolean;
  rich?: boolean;
}) {
  const { i18n } = useTranslation();
  const isEnglish = i18n.resolvedLanguage?.startsWith('en') ?? false;
  const scene = createScene(isEnglish);
  const [selectedId, setSelectedId] = useState(scene.persona.id);
  const [sent, setSent] = useState<JSONContent | null>(null);

  return (
    <div style={{ maxWidth: 850 }}>
      <ChatComposer
        personas={[scene.persona, scene.alternate]}
        selectedId={selectedId}
        onPersonaChange={setSelectedId}
        blocked={blocked}
        initialContent={rich ? scene.rich : undefined}
        onSend={setSent}
      />

      {sent && (
        <div style={{ padding: 24 }}>
          <RichMessage content={sent} />
        </div>
      )}
    </div>
  );
}

const meta = {
  title: 'Chat/Composer',
  component: ComposerPreview,
  parameters: {
    docs: {
      description: {
        component:
          'TipTap editor with bold, italic, underline, strike, headings, lists, quotes, code and links. Enter makes a paragraph; Ctrl/Cmd+Enter sends. During generation the draft remains editable while sending is disabled. The preview below echoes the submitted JSON through the message renderer.',
      },
    },
  },
} satisfies Meta<typeof ComposerPreview>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Empty: Story = {};

export const RichDraft: Story = { args: { rich: true } };

export const WhileCharacterReplies: Story = { args: { blocked: true } };

export const RepeatedUpdates: Story = {
  globals: { locale: 'en' },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const editor = await canvas.findByRole('textbox', { name: 'Message text' });
    const send = canvas.getByRole('button', { name: 'Send' });

    for (const text of ['First draft.', 'Second draft.', 'Third draft.']) {
      await userEvent.click(editor);
      const bold = canvas.getByRole('button', { name: 'Bold' });

      if (bold.getAttribute('aria-pressed') !== 'true') {
        await userEvent.click(bold);
      }
      await userEvent.type(editor, text);
      await expect(editor.querySelector('strong')).toHaveTextContent(text);
      await expect(send).toBeEnabled();
      await userEvent.click(send);
      await expect(editor).toHaveTextContent('');
      await expect(send).toBeDisabled();
      await expect(await canvas.findByText(text)).toBeInTheDocument();
    }
  },
};
