import { create } from './create';
import { remove } from './delete';
import { getAll } from './get-all';
import { update } from './update';
import { putAvatar } from './upload-avatar';

export { CharacterSchema } from './schema';
export type { Character } from './types';

export const characters = {
  create,
  getAll,
  update,
  remove,
  putAvatar,
};
