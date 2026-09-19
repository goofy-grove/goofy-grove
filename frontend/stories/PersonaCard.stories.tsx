import { expect, fn, userEvent, within } from 'storybook/test';

import { PersonaItem } from '@pages/personas/ui/persona-item';

import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = {
  title: 'Cards/Persona',
  component: PersonaItem,
  args: {
    uid: 'demo',
    name: 'Мох',
    description:
      'Хранитель рощи. Собирает потерянные истории и никогда не выходит из дома без термоса.',
    avatarUid: null,
    onEdit: fn(),
    onDelete: fn(),
  },
  decorators: [
    (Story) => (
      <div style={{ width: 'min(100%, 300px)' }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof PersonaItem>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const LongContent: Story = {
  args: {
    name: 'Очень длинное имя хранителя забытой рощи',
    description: 'Длинное описание с деталями характера и истории. '.repeat(12),
  },
};

export const WithoutDescription: Story = { args: { description: '' } };

export const DirectActions: Story = {
  play: async ({ canvasElement, args }) => {
    const buttons = within(canvasElement).getAllByRole('button');

    await expect(buttons).toHaveLength(2);
    await userEvent.click(buttons[0]);
    await expect(args.onEdit).toHaveBeenCalledWith(args.uid);
    await userEvent.click(buttons[1]);
    await expect(args.onDelete).toHaveBeenCalledWith(args.uid);
  },
};
