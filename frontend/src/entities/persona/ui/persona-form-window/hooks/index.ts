import { useState } from 'react';

import {
  useCreatePersonaMutation,
  useUpdatePersonaMutation,
} from '@entities/persona/model';
import type { PersonaFormWindowData } from '@entities/persona/ui/persona-form-window/types';

import { useCurrentWindow } from '@shared/ui';

export const usePersonaFormWindow = ({
  mode = 'create',
  uid,
  initialName = '',
  initialDescription = '',
}: PersonaFormWindowData) => {
  const { closeWindow } = useCurrentWindow();
  const [name, setName] = useState(initialName);
  const [description, setDescription] = useState(initialDescription);
  const createMutation = useCreatePersonaMutation();
  const updateMutation = useUpdatePersonaMutation();
  const mutation = mode === 'edit' ? updateMutation : createMutation;

  const handleSubmit = async () => {
    if (mode === 'edit') {
      if (!uid) {
        return;
      }

      await updateMutation.mutateAsync({ uid, name, description });
      closeWindow();
      return;
    }

    await createMutation.mutateAsync({ name, description });
    closeWindow();
  };

  return {
    mode,
    name,
    description,
    isPending: mutation.isPending,
    isError: mutation.isError,
    error: mutation.error,
    setName,
    setDescription,
    handleSubmit,
  };
};
