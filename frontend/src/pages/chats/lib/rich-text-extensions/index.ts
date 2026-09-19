import StarterKit from '@tiptap/starter-kit';

import type { RichTextExtensionsOptions } from './types';

export const richTextExtensions = (options?: RichTextExtensionsOptions) => [
  StarterKit.configure({
    heading: { levels: [2, 3] },
    link: {
      openOnClick: options?.readOnly,
      defaultProtocol: 'https',
      HTMLAttributes: { target: '_blank', rel: 'noopener noreferrer' },
    },
  }),
];
