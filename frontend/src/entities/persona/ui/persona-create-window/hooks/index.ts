import { useState } from 'react';

import { useCreatePersonaMutation } from '@entities/persona/model';

import { useCurrentWindow } from '@shared/ui';

export const usePersonaCreateWindow = () => {
  const { closeWindow } = useCurrentWindow();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const { isPending, mutateAsync, isError, error } = useCreatePersonaMutation();

  const handleCreate = async () => {
    await mutateAsync({ name, description });

    if (!isError) {
      closeWindow();
    }
  };

  return {
    name,
    description,
    isPending,
    isError,
    error,
    setName,
    setDescription,
    handleCreate,
  };
};
