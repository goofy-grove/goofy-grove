import { useModalSystem } from './use-modal-system';

export const useModal = () => {
  const { modals, addModal, removeModal, setActive } = useModalSystem();

  const openModal = (id: string) => {
    addModal({ id, isOpen: true, lastInteraction: Date.now() });
    setActive(id);
  };

  const closeModal = (id: string) => {
    removeModal(id);

    let lastInteraction = 0;
    let lastInteractionId = null;

    Object.values(modals).forEach((modal) => {
      if (modal.lastInteraction > lastInteraction) {
        lastInteractionId = modal.id;
        lastInteraction = modal.lastInteraction;
      }
    });

    setActive(lastInteractionId);
  };

  const isModalOpen = (id: string) => modals[id]?.isOpen;

  return {
    isModalOpen,
    openModal,
    closeModal,
  };
};
