import { create } from './create';
import { remove } from './delete';
import { getAll } from './get-all';
import { update } from './update';
import { putAvatar } from './upload-avatar';

export const characters = {
  create,
  getAll,
  update,
  remove,
  putAvatar,
};
