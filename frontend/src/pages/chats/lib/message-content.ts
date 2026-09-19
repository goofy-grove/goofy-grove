import { getSchema } from '@tiptap/react';

import { richTextExtensions } from './rich-text-extensions';

import type { JSONContent } from '@tiptap/react';

const schema = getSchema(richTextExtensions());

export const serializeMessageContent = (content: JSONContent): string =>
  JSON.stringify(content);

export const parseMessageContent = (value: string): JSONContent => {
  try {
    const parsed: unknown = JSON.parse(value);

    if (
      typeof parsed !== 'object' ||
      parsed === null ||
      !('type' in parsed) ||
      parsed.type !== 'doc'
    ) {
      throw new Error('Not a rich-text document');
    }
    const node = schema.nodeFromJSON(parsed);

    node.check();
    node.descendants((child) => {
      if (
        child.type.name === 'heading' &&
        ![2, 3].includes(Number(child.attrs.level))
      ) {
        throw new Error('Unsupported heading level');
      }
    });

    return node.toJSON() as JSONContent;
  } catch {
    return {
      type: 'doc',
      content: value.split(/\r?\n/).map((line) => ({
        type: 'paragraph',
        content: line ? [{ type: 'text', text: line }] : [],
      })),
    };
  }
};
