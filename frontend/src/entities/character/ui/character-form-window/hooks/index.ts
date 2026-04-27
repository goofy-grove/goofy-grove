import { useState } from 'react';

import {
  useCreateCharacterMutation,
  useUpdateCharacterMutation,
} from '@entities/character/model';
import type { CharacterFormWindowData } from '@entities/character/ui/character-form-window/types';

import { useCurrentWindow } from '@shared/ui';

export const useCharacterFormWindow = ({
  mode = 'create',
  uid,
  initialName = '',
  initialDescription = '',
}: CharacterFormWindowData) => {
  const { closeWindow } = useCurrentWindow();
  const [name, setName] = useState(initialName);
  const [description, setDescription] = useState(initialDescription);
  const createMutation = useCreateCharacterMutation();
  const updateMutation = useUpdateCharacterMutation();
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
