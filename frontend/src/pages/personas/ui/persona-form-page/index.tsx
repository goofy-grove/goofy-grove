import { useNavigate } from '@tanstack/react-router';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCreatePersonaMutation,
  usePersonasQuery,
  useUpdatePersonaMutation,
} from '@pages/personas/model';
import { PersonaForm } from '@pages/personas/ui/persona-form';

import { getApiErrorMessage } from '@shared/api';
import { Button, IconLoader, Text, useObjectUrl } from '@shared/ui';

import type { PersonaFormPageProps, PersonaFormStateProps } from './types';

import './styles.scss';

const PersonaFormState: FC<PersonaFormStateProps> = ({
  mode,
  uid,
  initialName,
  initialDescription,
  avatarUid,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const createMutation = useCreatePersonaMutation();
  const updateMutation = useUpdatePersonaMutation();
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

      void navigate({ to: '/personas' });
    } catch (error) {
      setErrorMessage(getApiErrorMessage(error, t));
    }
  };

  const isEditMode = mode === 'edit';

  return (
    <div className="persona-form-page">
      <div className="persona-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('persona.edit_title') : t('persona.create_title')}
        </Text>
        <Button
          variant="ghost"
          onClick={() => void navigate({ to: '/personas' })}
        >
          {t('common.back')}
        </Button>
      </div>

      <PersonaForm
        name={name}
        description={description}
        isPending={mutation.isPending}
        errorMessage={errorMessage}
        avatarUid={avatarUid}
        avatarPreviewUrl={avatarPreviewUrl}
        submitLabel={
          isEditMode ? t('persona.actions.edit') : t('persona.actions.create')
        }
        onNameChange={setName}
        onDescriptionChange={setDescription}
        onAvatarChange={setAvatarFile}
        onSubmit={() => void handleSubmit()}
      />
    </div>
  );
};

export const PersonaFormPage: FC<PersonaFormPageProps> = ({ mode, uid }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = usePersonasQuery();
  const persona =
    mode === 'edit' ? data?.find((item) => item.uid === uid) : null;

  if (mode === 'edit' && isLoading) {
    return (
      <div className="persona-form-page__loader">
        <IconLoader size={64} isAnimated />
      </div>
    );
  }

  if (mode === 'edit' && !persona) {
    return (
      <div className="persona-form-page">
        <Text>{t('persona.not_found')}</Text>
        <Button onClick={() => void navigate({ to: '/personas' })}>
          {t('common.back')}
        </Button>
      </div>
    );
  }

  if (mode === 'create') {
    return (
      <PersonaFormState
        key="create"
        mode="create"
        initialName=""
        initialDescription=""
      />
    );
  }

  return (
    <PersonaFormState
      key={persona!.uid}
      mode="edit"
      uid={persona!.uid}
      initialName={persona!.name}
      initialDescription={persona!.description}
      avatarUid={persona!.avatarUid}
    />
  );
};
