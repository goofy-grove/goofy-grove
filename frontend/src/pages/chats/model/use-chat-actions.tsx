import { IconPencil, IconTrash } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import type { ActionSheetItem } from '@shared/ui';

import { useDeleteChatMutation } from './mutations';

export const useChatActions = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [selectedElementUid, setSelectedElementUid] = useState<string | null>(
    null,
  );
  const [deletePendingUid, setDeletePendingUid] = useState<string | null>(null);
  const isActionsOpened = Boolean(selectedElementUid);
  const isDeleteConfirmationOpen = Boolean(deletePendingUid);

  const { isPending: isRemovePending, mutateAsync: removeChat } =
    useDeleteChatMutation();

  const handleOpen = (uid: string) => setSelectedElementUid(uid);

  const handleClose = () => setSelectedElementUid(null);

  const handleCloseDeleteConfirmation = () => setDeletePendingUid(null);

  const handleConfirmDeletion = async () => {
    if (!deletePendingUid) {
      return;
    }

    try {
      await removeChat({ uid: deletePendingUid });
    } finally {
      handleCloseDeleteConfirmation();
    }
  };

  const actions: ActionSheetItem[] = [
    {
      Icon: <IconPencil size={17} />,
      title: t('chat.actions.edit'),
      onClick: () => {
        if (selectedElementUid) {
          void navigate({
            to: '/chats/$uid',
            params: { uid: selectedElementUid },
          });
        }
      },
    },
    {
      Icon: <IconTrash size={17} />,
      title: t('chat.actions.delete'),
      color: 'error',
      onClick: () => {
        if (selectedElementUid) {
          setDeletePendingUid(selectedElementUid);
          setSelectedElementUid(null);
        }
      },
    },
  ];

  return {
    actions,
    isActionsOpened,
    isDeleteConfirmationOpen,
    handleOpen,
    handleClose,
    handleCloseDeleteConfirmation,
    handleConfirmDeletion,
    isRemovePending,
  };
};
