import { IconPlusFilled } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import {
  useDeletePersonaMutation,
  usePersonasQuery,
} from '@pages/personas/model';
import { PersonaItem } from '@pages/personas/ui/persona-item';

import {
  Button,
  ConfirmModal,
  GroveScene,
  IconLoader,
  PageHeader,
  Text,
} from '@shared/ui';

import './styles.scss';

export const PersonasPage: FC = () => {
  const { data, isLoading } = usePersonasQuery();
  const deletePersona = useDeletePersonaMutation();
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [pendingDeleteUid, setPendingDeleteUid] = useState<string | null>(null);

  const hasPersonas = !isLoading && !!data?.length;

  const handleEdit = (uid: string) => {
    void navigate({ to: '/personas/$uid', params: { uid } });
  };

  const handleDelete = (uid: string) => {
    setPendingDeleteUid(uid);
  };

  const handleCancelDelete = () => {
    setPendingDeleteUid(null);
  };

  const handleConfirmDelete = () => {
    if (!pendingDeleteUid) {
      return;
    }

    void deletePersona
      .mutateAsync({ uid: pendingDeleteUid })
      .then(() => setPendingDeleteUid(null))
      .catch(() => undefined);
  };

  return (
    <div className="personas-page">
      <PageHeader title={t('persona.list_title')}>
        <Button
          leftIcon={<IconPlusFilled size={18} />}
          onClick={() => void navigate({ to: '/personas/new' })}
        >
          {t('persona.create_title')}
        </Button>
      </PageHeader>

      <p className="personas-page__intro">{t('grove.personas_intro')}</p>

      {isLoading && (
        <div className="personas-page__loader">
          <IconLoader size={64} isAnimated />
        </div>
      )}

      {!isLoading && !hasPersonas && (
        <div className="personas-page__empty">
          <GroveScene />

          <Text tag="h3">{t('persona.empty')}</Text>

          <Text>{t('grove.personas_empty')}</Text>
        </div>
      )}

      <div className="personas-page__list scrollbar">
        {hasPersonas &&
          data.map((persona) => (
            <PersonaItem
              uid={persona.uid}
              name={persona.name}
              description={persona.description}
              avatarUid={persona.avatar_uid}
              key={persona.uid}
              onDelete={handleDelete}
              onEdit={handleEdit}
            />
          ))}
      </div>

      <ConfirmModal
        isOpen={pendingDeleteUid !== null}
        message={t('persona.confirm_delete')}
        confirmLabel={t('persona.actions.delete')}
        cancelLabel={t('common.cancel')}
        isPending={deletePersona.isPending}
        onConfirm={handleConfirmDelete}
        onCancel={handleCancelDelete}
      />
    </div>
  );
};
