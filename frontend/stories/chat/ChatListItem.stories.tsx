import { expect, fn, userEvent, within } from 'storybook/test';

import { toChatSummary } from '@pages/chats/model';
import { ChatListItem } from '@pages/chats/ui/chat-list-item';

import forest from './assets/forest.svg';

import type { Meta, StoryObj } from '@storybook/react-vite';

const summary = toChatSummary({
  uid: 'lighthouse',
  name: 'Ночь у старого маяка',
  created_at: new Date('2026-09-16T12:40:00+03:00'),
  creator_uid: 'me',
  avatar_uid: null,
  members: [],
  characters: [
    {
      uid: 'owl',
      name: 'Сова',
      description: '',
      creator_uid: 'me',
      avatar_uid: null,
      chat_uid: 'lighthouse',
      connected_at: new Date('2026-09-16T12:40:00+03:00'),
    },
    {
      uid: 'moss',
      name: 'Мох',
      description: '',
      creator_uid: 'me',
      avatar_uid: null,
      chat_uid: 'lighthouse',
      connected_at: new Date('2026-09-16T12:40:00+03:00'),
    },
  ],
});

const meta = {
  title: 'Chat/List item',
  component: ChatListItem,
  args: {
    uid: summary.uid,
    title: summary.title,
    avatarUid: summary.avatarUid,
    imageUrl: forest,
    characters: summary.characters,
    lastMessage: {
      author: 'Сова',
      text: 'Кажется, за дверью кто-то есть. Ты тоже слышишь эти шаги?',
    },
    activity: { label: '12:40', dateTime: '2026-09-16T12:40:00+03:00' },
    unreadCount: 0,
    onOpen: fn(),
    onActions: fn(),
  },
  decorators: [
    (Story) => (
      <div style={{ width: 'min(100%, 820px)' }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof ChatListItem>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Unread: Story = { args: { unreadCount: 3 } };

export const Generating: Story = {
  args: { generatingCharacterName: 'Сова', unreadCount: 2 },
};

export const FirstMessage: Story = {
  args: { lastMessage: undefined, activity: undefined, imageUrl: undefined },
};

export const WithoutCharacters: Story = {
  args: { characters: [], imageUrl: undefined },
};

export const WithoutActions: Story = { args: { onActions: undefined } };

export const BrokenImage: Story = {
  args: { imageUrl: '/missing-chat-scene.png' },
};

export const LongContent: Story = {
  args: {
    title:
      'Очень длинная история о том, как хранители рощи отправились к забытому маяку',
    lastMessage: {
      author: 'Хранитель далёкой северной рощи',
      text: 'На пороге лежало письмо. Никто не знал, откуда оно взялось, но каждый узнавал почерк. '.repeat(
        4,
      ),
    },
    unreadCount: 128,
    characters: Array.from({ length: 8 }, (_, index) => ({
      id: `character-${index}`,
      name: `Персонаж ${index + 1}`,
      initials: String(index + 1),
    })),
  },
};

export const Mobile: Story = {
  args: { unreadCount: 3 },
  globals: { viewport: { value: 'mobile1', isRotated: false } },
  parameters: {
    viewport: {
      options: {
        mobile1: {
          name: 'Small phone',
          styles: { width: '320px', height: '640px' },
          type: 'mobile',
        },
      },
    },
  },
};

export const English: Story = {
  globals: { locale: 'en' },
  args: {
    title: 'A night at the old lighthouse',
    generatingCharacterName: 'Owl',
    unreadCount: 3,
    characters: [
      { id: 'owl', name: 'Owl', initials: 'O' },
      { id: 'moss', name: 'Moss', initials: 'M' },
    ],
  },
};

export const InList: Story = {
  render: (args) => (
    <div style={{ display: 'grid', gap: 12 }}>
      <ChatListItem {...args} unreadCount={3} />

      <ChatListItem
        {...args}
        uid="observatory"
        title="Там, где светятся грибы"
        imageUrl={undefined}
        generatingCharacterName="Мох"
        activity={{ label: '12:38', dateTime: '2026-09-16T12:38:00+03:00' }}
      />

      <ChatListItem
        {...args}
        uid="first-story"
        title="Дом на краю леса"
        imageUrl={undefined}
        lastMessage={undefined}
        activity={{ label: 'Вчера', dateTime: '2026-09-15' }}
      />
    </div>
  ),
};

export const OpenAndActions: Story = {
  play: async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    const open = canvas.getByRole('button', { name: args.title });
    const actions = canvas.getByRole('button', {
      name: `Действия: ${args.title}`,
    });

    await userEvent.click(actions);
    await expect(args.onActions).toHaveBeenCalledWith(args.uid);
    await expect(args.onOpen).not.toHaveBeenCalled();
    open.focus();
    await userEvent.keyboard('{Enter}');
    await expect(args.onOpen).toHaveBeenCalledWith(args.uid);
  },
};
