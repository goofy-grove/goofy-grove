import { expect, fireEvent, userEvent, waitFor, within } from 'storybook/test';
import { MINIMAL_VIEWPORTS } from 'storybook/viewport';

import { ChatPreview } from './chat-preview';

import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = {
  title: 'Chat/Conversation',
  component: ChatPreview,
  parameters: {
    layout: 'fullscreen',
    viewport: { options: MINIMAL_VIEWPORTS },
    docs: {
      description: {
        component:
          'Reusable shared chat components with a Storybook-only local demo. Humans write through their own Persona; AI replies through a Character. Composer and messages share a TipTap JSON schema. Sending, editing, retry and stop run locally. The Character response is a fixed demo, not an AI call. No attachments or application routes.',
      },
    },
  },
  decorators: [
    (Story) => (
      <div style={{ maxWidth: 980, margin: '0 auto' }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof ChatPreview>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Conversation: Story = {};

export const FirstMessage: Story = { args: { state: 'empty' } };

export const CharacterIsThinking: Story = { args: { state: 'generating' } };

export const FailedMessage: Story = { args: { state: 'failed' } };

export const ReadOnly: Story = { args: { state: 'read-only' } };

export const Mobile: Story = {
  globals: { viewport: { value: 'mobile1', isRotated: false } },
};

export const SendFormattedMessage: Story = {
  args: { state: 'empty' },
  globals: { locale: 'en' },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const send = canvas.getByRole('button', { name: 'Send' });

    await expect(send).toBeDisabled();
    const editor = await canvas.findByRole('textbox', { name: 'Message text' });

    await userEvent.click(editor);
    await userEvent.click(canvas.getByRole('button', { name: 'Bold' }));
    await userEvent.type(editor, 'A lantern in the rain.');
    await userEvent.click(send);
    const message = await canvas.findByRole('article', { name: 'Fern' });

    await expect(message.querySelector('strong')).toHaveTextContent(
      'A lantern in the rain.',
    );
    await expect(within(message).getByText('Persona')).toBeInTheDocument();
    await expect(editor.textContent).toBe('');
    await expect(send).toBeDisabled();
    await userEvent.click(await canvas.findByRole('button', { name: 'Stop' }));
    await expect(canvas.getByRole('status')).toHaveTextContent('Reply stopped');
  },
};

export const ChoosePersona: Story = {
  args: { state: 'empty' },
  globals: { locale: 'en' },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);

    await userEvent.click(
      canvas.getByRole('button', { name: 'Choose your Persona' }),
    );
    await expect(
      page.queryByRole('option', { name: 'Moss' }),
    ).not.toBeInTheDocument();
    await expect(
      page.queryByRole('option', { name: 'Clover' }),
    ).not.toBeInTheDocument();
    await userEvent.click(await page.findByRole('option', { name: 'Robin' }));
    await userEvent.type(
      await canvas.findByRole('textbox', { name: 'Message text' }),
      'Is anyone home?',
    );
    await userEvent.click(canvas.getByRole('button', { name: 'Send' }));
    await expect(
      await canvas.findByRole('article', { name: 'Robin' }),
    ).toHaveTextContent('Is anyone home?');
    await userEvent.click(await canvas.findByRole('button', { name: 'Stop' }));
  },
};

export const CustomBackground: Story = { args: { background: true } };

export const MobileWithBackground: Story = {
  args: { background: true },
  globals: { viewport: { value: 'mobile1', isRotated: false } },
};

export const PagedHistory: Story = {
  args: { state: 'history' },
  parameters: {
    docs: {
      description: {
        story:
          '250 messages. Initially 50 are loaded; scroll to the top to load older pages of 50. Only the viewport and overscan are mounted. Prepending preserves the visible message; appended replies follow only when already at the bottom.',
      },
    },
  },
};

export const HistoryLoadError: Story = { args: { state: 'history-error' } };

export const TriggerSpecificCharacter: Story = {
  globals: { locale: 'en' },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    await expect(
      canvas.queryByRole('button', { name: 'Edit character: Owl' }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole('button', { name: 'Edit character: Moss' }),
    ).toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole('button', { name: 'Ask to reply: Owl' }),
    );
    await expect(await canvas.findByRole('status')).toHaveTextContent(
      'Owl is thinking',
    );
    await userEvent.click(canvas.getByRole('button', { name: 'Stop' }));
  },
};

export const LoadOlderPage: Story = {
  args: { state: 'history' },
  globals: { locale: 'en' },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const timeline = canvas.getByRole('region', {
      name: 'Earlier in the story',
    });

    await waitFor(() =>
      expect(
        timeline.querySelector('[data-message-id="history-249"]'),
      ).toBeInTheDocument(),
    );
    await expect(
      timeline.querySelectorAll('[data-message-id]').length,
    ).toBeLessThan(50);
    timeline.scrollTop = 0;
    await fireEvent.scroll(timeline);
    await expect(await canvas.findByRole('status')).toHaveTextContent(
      'Turning back the pages',
    );
    await waitFor(
      () =>
        expect(
          canvas.queryByText('Turning back the pages…'),
        ).not.toBeInTheDocument(),
      { timeout: 5000 },
    );
    timeline.scrollTop = 0;
    await fireEvent.scroll(timeline);
    await waitFor(() =>
      expect(
        timeline.querySelector('[data-message-id="history-150"]'),
      ).toBeInTheDocument(),
    );
    await expect(
      timeline.querySelectorAll('[data-message-id]').length,
    ).toBeLessThan(50);
  },
};
