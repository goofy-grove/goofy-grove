import { IconPencil, IconTrash } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { useBreakpoints } from '@shared/ui';
import type { ActionSheetItem } from '@shared/ui/action-sheet/types';

import { useDeleteCharacterMutation } from './mutations';

export const useCharacterActions = () => {
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [pendingDeleteUid, setPendingDeleteUid] = useState<string | null>(null);
  const [selectedCharacterUid, setSelectedCharacterUid] = useState<
    string | null
  >(null);

  const { t } = useTranslation();
  const actions: ActionSheetItem[] = [
    {
      Icon: <IconPencil size={17} />,
      title: t('character.actions.edit'),
      onClick: () => {
        if (selectedCharacterUid) {
          handleEdit(selectedCharacterUid);
        }
      },
    },
    {
      Icon: <IconTrash size={17} />,
      title: t('character.actions.delete'),
      onClick: () => {
        if (selectedCharacterUid) {
          handleDelete(selectedCharacterUid);
        }
      },
      color: 'error',
    },
  ];

  const { isMobileSm } = useBreakpoints();
  const deleteCharacter = useDeleteCharacterMutation();

  const navigate = useNavigate();

  const handleOpenActionSheet = (uid: string) => {
    if (isMobileSm) {
      setSelectedCharacterUid(uid);
      setIsActionSheetOpen(true);
    }
  };

  const handleCloseActionSheet = () => {
    if (isMobileSm) {
      setIsActionSheetOpen(false);
      setSelectedCharacterUid(null);
    }
  };

  const handleEdit = (uid: string) => {
    void navigate({ to: '/characters/$uid', params: { uid } });
  };

  const handleDelete = (uid: string) => {
    setIsActionSheetOpen(false);
    setPendingDeleteUid(uid);
  };

  const handleCancelDelete = () => {
    setPendingDeleteUid(null);

    handleCloseActionSheet();
  };

  const handleConfirmDelete = async () => {
    if (!pendingDeleteUid) {
      return;
    }

    try {
      await deleteCharacter.mutateAsync({ uid: pendingDeleteUid });
    } finally {
      setPendingDeleteUid(null);
      handleCloseActionSheet();
    }
  };

  return {
    actions,
    isActionSheetOpen,
    pendingDeleteUid,
    deleteCharacter,

    handleDelete,
    handleCancelDelete,
    handleConfirmDelete,
    handleEdit,
    handleOpenActionSheet,
    handleCloseActionSheet,
  };
};
