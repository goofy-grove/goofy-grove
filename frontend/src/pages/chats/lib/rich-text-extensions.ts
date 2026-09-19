import StarterKit from '@tiptap/starter-kit';

export const richTextExtensions = (readOnly = false) => [
  StarterKit.configure({
    heading: { levels: [2, 3] },
    link: {
      openOnClick: readOnly,
      defaultProtocol: 'https',
      HTMLAttributes: { target: '_blank', rel: 'noopener noreferrer' },
    },
  }),
];
