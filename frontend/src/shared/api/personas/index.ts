import { create } from './create';
import { remove } from './delete';
import { getAll } from './get-all';
import { update } from './update';
import { putAvatar } from './upload-avatar';

export { PersonaSchema } from './schema';
export type { Persona } from './types';

export const personas = {
  create,
  getAll,
  update,
  remove,
  putAvatar,
};
