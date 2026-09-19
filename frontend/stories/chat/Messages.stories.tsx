import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { ChatMessage } from '@pages/chats/ui/chat-message';

import { createScene } from './model';

import type { Meta, StoryObj } from '@storybook/react-vite';
import type { JSONContent } from '@tiptap/react';

function MessagePreview({
  kind = 'character',
  rich = false,
  failed = false,
}: {
  kind?: 'persona' | 'character' | 'companion';
  rich?: boolean;
  failed?: boolean;
}) {
  const { i18n } = useTranslation();
  const isEnglish = i18n.resolvedLanguage?.startsWith('en') ?? false;
  const scene = createScene(isEnglish);
  const [edited, setEdited] = useState<JSONContent>();
  const [retried, setRetried] = useState(false);
  const message =
    scene.messages[kind === 'persona' ? 1 : kind === 'companion' ? 2 : 0];

  return (
    <ChatMessage
      message={{
        ...message,
        content: edited ?? (rich ? scene.rich : message.content),
        edited: !!edited,
        status: failed && !retried ? 'failed' : 'sent',
      }}
      onEdit={(_, content) => setEdited(content)}
      onRetry={() => setRetried(true)}
    />
  );
}

const meta = {
  title: 'Chat/Messages',
  component: MessagePreview,
  decorators: [
    (Story) => (
      <div style={{ maxWidth: 760 }}>
        <Story />
      </div>
    ),
  ],
  parameters: {
    docs: {
      description: {
        component:
          'Read-only TipTap rendering with the same extensions as the composer. Persona identifies a human role; Character identifies the AI role. Only your own Persona messages offer editing.',
      },
    },
  },
} satisfies Meta<typeof MessagePreview>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Character: Story = {};

export const YourPersona: Story = { args: { kind: 'persona' } };

export const OtherPersona: Story = { args: { kind: 'companion' } };

export const RichText: Story = { args: { rich: true } };

export const Failed: Story = { args: { kind: 'persona', failed: true } };
