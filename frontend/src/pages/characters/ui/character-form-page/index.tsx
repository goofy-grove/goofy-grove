import { IconArrowLeft } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCreateCharacterMutation,
  useCharactersQuery,
  useUpdateCharacterMutation,
} from '@pages/characters/model';
import { CharacterForm } from '@pages/characters/ui/character-form';
import type { CharacterFormValues } from '@pages/characters/ui/character-form/types';

import { getApiErrorMessage } from '@shared/api';
import { Button, IconLoader, Text } from '@shared/ui';

import type { CharacterFormPageProps } from './types';

import './styles.scss';

export const CharacterFormPage: FC<CharacterFormPageProps> = ({
  mode,
  uid,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = useCharactersQuery();
  const createMutation = useCreateCharacterMutation();
  const updateMutation = useUpdateCharacterMutation();
  const isEditMode = mode === 'edit';
  const mutation = isEditMode ? updateMutation : createMutation;

  const handleBack = () => void navigate({ to: '/characters' });

  const handleSubmit = async (values: CharacterFormValues) => {
    if (isEditMode) {
      if (!uid) {
        return;
      }

      await updateMutation.mutateAsync({ uid, ...values });
    } else {
      await createMutation.mutateAsync(values);
    }

    handleBack();
  };

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
      <div className="character-form-page scrollbar">
        <Text>{t('character.not_found')}</Text>

        <Button onClick={() => void navigate({ to: '/characters' })}>
          {t('common.back')}
        </Button>
      </div>
    );
  }

  return (
    <div className="character-form-page scrollbar">
      <div className="character-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('character.edit_title') : t('character.create_title')}
        </Text>

        <Button
          variant="ghost"
          disabled={mutation.isPending}
          leftIcon={<IconArrowLeft size={18} />}
          onClick={handleBack}
        >
          {t('common.back')}
        </Button>
      </div>

      <p className="character-form-page__intro">{t('forms.character_intro')}</p>

      <CharacterForm
        key={isEditMode ? character!.uid : 'create'}
        initialValues={{
          name: character?.name ?? '',
          description: character?.description ?? '',
        }}
        avatarUid={character?.avatar_uid || undefined}
        isPending={mutation.isPending}
        errorMessage={
          mutation.isError ? getApiErrorMessage(mutation.error, t) : undefined
        }
        submitLabel={
          isEditMode ? t('forms.save_changes') : t('character.create_title')
        }
        onSubmit={handleSubmit}
        onCancel={handleBack}
      />
    </div>
  );
};
