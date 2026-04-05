import { useCallback } from 'react';

import { useModalSystem } from './use-modal-system';

export const useModal = () => {
  const { modals, addModal, removeModal, setActive } = useModalSystem();

  const openModal = useCallback(
    (id: string) => {
      if (modals.current[id]?.isOpen) {
        return;
      }

      addModal({ id, isOpen: true, lastInteraction: Date.now() });
      setActive(id);
    },
    [modals, addModal, setActive],
  );

  const closeModal = useCallback(
    (id: string) => {
      removeModal(id);

      let lastInteraction = 0;
      let lastInteractionId = null;

      Object.values(modals.current).forEach((modal) => {
        if (modal.lastInteraction > lastInteraction) {
          lastInteractionId = modal.id;
          lastInteraction = modal.lastInteraction;
        }
      });

      setActive(lastInteractionId);
    },
    [modals, removeModal, setActive],
  );

  const isModalOpen = useCallback(
    (id: string) => modals.current[id]?.isOpen,
    [modals],
  );

  return {
    isModalOpen,
    openModal,
    closeModal,
  };
};
