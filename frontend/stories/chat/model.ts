import {
  toChatPersona,
  toChatCharacter,
  toChatMessage,
  toSendMessage,
} from '@pages/chats/model';
import type { ChatMessageData } from '@pages/chats/model';

import type { MessageAuthorDto } from '@shared/api/messages';

import type { JSONContent } from '@tiptap/react';

export const text = (value: string, mark?: string): JSONContent => ({
  type: 'text',
  text: value,
  ...(mark ? { marks: [{ type: mark }] } : {}),
});

export const paragraph = (...content: JSONContent[]): JSONContent => ({
  type: 'paragraph',
  content,
});

export const document = (...content: JSONContent[]): JSONContent => ({
  type: 'doc',
  content,
});

export function createScene(en: boolean) {
  const context = {
    currentUserUid: 'me',
    locale: en ? 'en' : 'ru',
    ownerNames: {
      me: en ? 'You' : 'Вы',
      sasha: en ? 'Sasha' : 'Саша',
    },
  };
  const personaDto = {
    uid: 'fern',
    name: en ? 'Fern' : 'Папоротник',
    description: '',
    creator_uid: 'me',
    avatar_uid: null,
  };
  const alternateDto = {
    ...personaDto,
    uid: 'robin',
    name: en ? 'Robin' : 'Зарянка',
  };
  const companionDto = {
    ...personaDto,
    uid: 'clover',
    name: en ? 'Clover' : 'Клевер',
    creator_uid: 'sasha',
  };
  const characterDto = {
    uid: 'moss',
    name: en ? 'Moss' : 'Мох',
    description: '',
    creator_uid: 'me',
    avatar_uid: null,
  };
  const persona = toChatPersona(personaDto, context);
  const alternate = toChatPersona(alternateDto, context);
  const companion = toChatPersona(companionDto, context);
  const character = toChatCharacter(characterDto);
  const authors: Record<string, MessageAuthorDto> = {
    fern: { ...personaDto, kind: 'persona' },
    clover: { ...companionDto, kind: 'persona' },
    moss: { ...characterDto, kind: 'character' },
  };
  const opening = document(
    paragraph(
      text(
        en
          ? 'The last lantern on the path flickered. Moss leaned out of a hollow tree, carrying a kettle that was quietly whistling to itself.'
          : 'Последний фонарь на тропе мигнул. Мох выглянул из дупла с чайником, который тихонько насвистывал сам себе.',
        'italic',
      ),
    ),
    paragraph(
      text(
        en
          ? '“Well, come in. The forest only loses people who are in a hurry.”'
          : '— Ну, заходите. Лес теряет только тех, кто слишком торопится.',
      ),
    ),
    {
      type: 'blockquote',
      content: [
        paragraph(
          text(
            en
              ? 'Beyond the doorway: warm light, dry socks, and the smell of wild mint.'
              : 'За порогом — тёплый свет, сухие носки и запах лесной мяты.',
          ),
        ),
      ],
    },
  );
  const reply = document(
    paragraph(
      text(
        en
          ? 'Moss set three cups on the table. One of them had already begun to snore.'
          : 'Мох поставил на стол три чашки. Одна из них уже начала похрапывать.',
        'italic',
      ),
    ),
    paragraph(
      text(
        en
          ? '“A map? Of course. But first, promise me one thing: '
          : '— Карта? Конечно. Но сначала пообещайте мне одну вещь: ',
      ),
      text(
        en ? 'do not follow the blue mushrooms' : 'не идите за синими грибами',
        'bold',
      ),
      text(
        en
          ? '. They have a terrible sense of direction.”'
          : '. У них ужасно с ориентированием.',
      ),
    ),
  );
  const messageDrafts: ChatMessageData[] = [
    { id: 'm1', author: character, content: opening, time: '21:04' },
    {
      id: 'm2',
      author: persona,
      time: '21:05',
      content: document(
        paragraph(
          text(
            en
              ? 'I shake the rain from my cloak and stop at the doorway.'
              : 'Стряхиваю дождь с плаща и останавливаюсь на пороге.',
            'italic',
          ),
        ),
        paragraph(
          text(
            en
              ? '“We were looking for the old observatory. Do you happen to have a '
              : '— Мы искали старую обсерваторию. У вас случайно нет ',
          ),
          text(en ? 'map' : 'карты', 'bold'),
          text(en ? '?”' : '?'),
        ),
      ),
    },
    {
      id: 'm3',
      author: companion,
      time: '21:06',
      content: document(
        paragraph(
          text(
            en
              ? '“And maybe a little tea? Asking for a friend.”'
              : '— И, может быть, немного чая? Друг интересуется.',
          ),
        ),
      ),
    },
    { id: 'm4', author: character, content: reply, time: '21:07' },
  ];
  const messages = messageDrafts.map((message) => {
    const author = message.author ? authors[message.author.id] : null;
    const content = author
      ? toSendMessage(message.content, {
          kind: author.kind,
          uid: author.uid,
        }).content
      : '';

    return toChatMessage(
      {
        uid: message.id,
        author,
        content,
        chat_uid: 'grove',
        created_at: new Date(`2026-09-18T${message.time}:00`),
        reply_to_message: null,
        is_removed: false,
      },
      context,
    );
  });
  const rich = document(
    {
      type: 'heading',
      attrs: { level: 2 },
      content: [text(en ? 'A note in the margin' : 'Заметка на полях')],
    },
    paragraph(
      text(en ? 'The ink smells of ' : 'Чернила пахнут '),
      text(en ? 'pine needles' : 'хвоей', 'italic'),
      text(en ? '. Someone has underlined ' : '. Кто-то подчеркнул '),
      text(en ? 'the important part' : 'самое важное', 'underline'),
      text('.'),
    ),
    {
      type: 'bulletList',
      content: [
        en ? 'Bring a lantern.' : 'Взять фонарь.',
        en ? 'Leave the blue mushrooms alone.' : 'Не трогать синие грибы.',
      ].map((line) => ({ type: 'listItem', content: [paragraph(text(line))] })),
    },
    {
      type: 'orderedList',
      content: [
        en ? 'Cross the bridge.' : 'Перейти мост.',
        en ? 'Knock twice.' : 'Постучать дважды.',
      ].map((line) => ({ type: 'listItem', content: [paragraph(text(line))] })),
    },
    {
      type: 'blockquote',
      content: [
        paragraph(
          text(en ? 'Every path has a story.' : 'У каждой тропы есть история.'),
        ),
      ],
    },
    paragraph(
      text(en ? 'Password: ' : 'Пароль: '),
      text('lumen', 'code'),
      text('. '),
      text(en ? 'Forget it.' : 'Забыть.', 'strike'),
    ),
    { type: 'codeBlock', content: [text('north → bridge → grove')] },
    paragraph({
      type: 'text',
      text: en ? 'About constellations' : 'О созвездиях',
      marks: [
        {
          type: 'link',
          attrs: {
            href: 'https://science.nasa.gov/universe/stars/constellations/',
          },
        },
      ],
    }),
  );

  return { persona, alternate, companion, character, messages, reply, rich };
}
