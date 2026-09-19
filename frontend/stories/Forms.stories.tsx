import { useState } from 'react';
import { fn } from 'storybook/test';

import { CharacterForm } from '@pages/characters/ui/character-form';

import { useObjectUrl } from '@shared/ui/hooks';

import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = {
  title: 'Forms/Character',
  component: CharacterForm,
  args: {
    name: '',
    description: '',
    isPending: false,
    submitLabel: 'Создать персонажа',
    onNameChange: fn(),
    onDescriptionChange: fn(),
    onAvatarChange: fn(),
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

export const Create: Story = {
  render: function Create(args) {
    const [name, setName] = useState(args.name);
    const [description, setDescription] = useState(args.description);
    const [avatar, setAvatar] = useState<File | null>(null);
    const preview = useObjectUrl(avatar);

    return (
      <CharacterForm
        {...args}
        name={name}
        description={description}
        avatarPreviewUrl={preview}
        onAvatarChange={setAvatar}
        onNameChange={setName}
        onDescriptionChange={setDescription}
      />
    );
  },
};

export const Saving: Story = {
  args: { name: 'Мох', isPending: true, submitLabel: 'Сохранить изменения' },
};

export const Error: Story = {
  args: {
    name: 'Мох',
    errorMessage: 'Не удалось сохранить. Попробуйте ещё раз.',
  },
};
