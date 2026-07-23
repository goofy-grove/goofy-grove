import { useNavigate } from '@tanstack/react-router';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCharactersQuery,
  useCreateCharacterMutation,
  useUpdateCharacterMutation,
} from '@pages/characters/model';
import { CharacterForm } from '@pages/characters/ui/character-form';

import { Button, IconLoader, Text, useObjectUrl } from '@shared/ui';

import type { CharacterFormPageProps, CharacterFormStateProps } from './types';

import './styles.scss';

const CharacterFormState: FC<CharacterFormStateProps> = ({
  mode,
  uid,
  initialName,
  initialDescription,
  avatarUid,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const createMutation = useCreateCharacterMutation();
  const updateMutation = useUpdateCharacterMutation();
  const mutation = mode === 'edit' ? updateMutation : createMutation;

  const [name, setName] = useState(initialName);
  const [description, setDescription] = useState(initialDescription);
  const [avatarFile, setAvatarFile] = useState<File | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const avatarPreviewUrl = useObjectUrl(avatarFile);

  const handleSubmit = async () => {
    setErrorMessage(null);

    try {
      if (mode === 'edit') {
        if (!uid) {
          return;
        }

        await updateMutation.mutateAsync({
          uid,
          name,
          description,
          avatarFile,
        });
      } else {
        await createMutation.mutateAsync({
          name,
          description,
          avatarFile,
        });
      }

      void navigate({ to: '/characters' });
    } catch {
      setErrorMessage(t('common.errors.save_failed'));
    }
  };

  const isEditMode = mode === 'edit';

  return (
    <div className="character-form-page">
      <div className="character-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('character.edit_title') : t('character.create_title')}
        </Text>
        <Button
          variant="ghost"
          onClick={() => void navigate({ to: '/characters' })}
        >
          {t('common.back')}
        </Button>
      </div>

      <CharacterForm
        name={name}
        description={description}
        isPending={mutation.isPending}
        errorMessage={errorMessage}
        avatarUid={avatarUid}
        avatarPreviewUrl={avatarPreviewUrl}
        submitLabel={
          isEditMode
            ? t('character.actions.edit')
            : t('character.actions.create')
        }
        onNameChange={setName}
        onDescriptionChange={setDescription}
        onAvatarChange={setAvatarFile}
        onSubmit={() => void handleSubmit()}
      />
    </div>
  );
};

export const CharacterFormPage: FC<CharacterFormPageProps> = ({
  mode,
  uid,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = useCharactersQuery();
  const character =
    mode === 'edit' ? data?.find((item) => item.uid === uid) : null;

  if (mode === 'edit' && isLoading) {
    return (
      <div className="character-form-page__loader">
        <IconLoader size={64} isAnimated />
      </div>
    );
  }

  if (mode === 'edit' && !character) {
    return (
      <div className="character-form-page">
        <Text>{t('character.not_found')}</Text>
        <Button onClick={() => void navigate({ to: '/characters' })}>
          {t('common.back')}
        </Button>
      </div>
    );
  }

  if (mode === 'create') {
    return (
      <CharacterFormState
        key="create"
        mode="create"
        initialName=""
        initialDescription=""
      />
    );
  }

  return (
    <CharacterFormState
      key={character!.uid}
      mode="edit"
      uid={character!.uid}
      initialName={character!.name}
      initialDescription={character!.description}
      avatarUid={character!.avatarUid}
    />
  );
};
