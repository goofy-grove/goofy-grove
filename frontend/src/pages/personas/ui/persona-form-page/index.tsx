import { IconArrowLeft } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useCreatePersonaMutation,
  usePersonasQuery,
  useUpdatePersonaMutation,
} from '@pages/personas/model';
import { PersonaForm } from '@pages/personas/ui/persona-form';
import type { PersonaFormValues } from '@pages/personas/ui/persona-form/types';

import { getApiErrorMessage } from '@shared/api';
import { Button, IconLoader, Text } from '@shared/ui';

import type { PersonaFormPageProps } from './types';

import './styles.scss';

export const PersonaFormPage: FC<PersonaFormPageProps> = ({ mode, uid }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = usePersonasQuery();
  const createMutation = useCreatePersonaMutation();
  const updateMutation = useUpdatePersonaMutation();
  const isEditMode = mode === 'edit';
  const mutation = isEditMode ? updateMutation : createMutation;

  const handleBack = () => void navigate({ to: '/personas' });
  const handleSubmit = async (values: PersonaFormValues) => {
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
      <div className="persona-form-page scrollbar">
        <Text>{t('persona.not_found')}</Text>

        <Button onClick={() => void navigate({ to: '/personas' })}>
          {t('common.back')}
        </Button>
      </div>
    );
  }

  return (
    <div className="persona-form-page scrollbar">
      <div className="persona-form-page__header">
        <Text tag="h2">
          {isEditMode ? t('persona.edit_title') : t('persona.create_title')}
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

      <p className="persona-form-page__intro">{t('forms.persona_intro')}</p>

      <PersonaForm
        key={isEditMode ? persona!.uid : 'create'}
        initialValues={{
          name: persona?.name ?? '',
          description: persona?.description ?? '',
        }}
        avatarUid={persona?.avatar_uid || undefined}
        isPending={mutation.isPending}
        errorMessage={
          mutation.isError ? getApiErrorMessage(mutation.error, t) : undefined
        }
        submitLabel={
          isEditMode ? t('forms.save_changes') : t('persona.create_title')
        }
        onSubmit={handleSubmit}
        onCancel={handleBack}
      />
    </div>
  );
};
