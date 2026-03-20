import { api } from '../axios';
import { withAuth, withValidation } from '../common';
import { PersonSchema } from './schema';

export const create = withAuth(
  withValidation(PersonSchema, async (name: string, description: string) => {
    const response = await api.post('/persons', { name, description });

    return response.data;
  }),
);
