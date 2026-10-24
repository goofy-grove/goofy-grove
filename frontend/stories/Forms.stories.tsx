import { fn } from 'storybook/test';

import { CharacterForm } from '@pages/characters/ui/character-form';

import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = {
  title: 'Forms/Character',
  component: CharacterForm,
  args: {
    initialValues: { name: '', description: '' },
    isPending: false,
    submitLabel: 'Создать персонажа',
    onSubmit: fn(),
    onCancel: fn(),
  },
  decorators: [
    (Story) => (
      <div style={{ maxWidth: 920 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof CharacterForm>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Create: Story = {};

export const Saving: Story = {
  args: {
    initialValues: { name: 'Мох', description: '' },
    isPending: true,
    submitLabel: 'Сохранить изменения',
  },
};

export const Error: Story = {
  args: {
    initialValues: { name: 'Мох', description: '' },
    errorMessage: 'Не удалось сохранить. Попробуйте ещё раз.',
  },
};
