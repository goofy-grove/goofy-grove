import { useEffect, useRef, useState, type FC } from 'react';

import {
  ModalContext,
  type ModalState,
  type ModalWindowContext,
} from './hooks';

import type { ModalProviderProps } from './types';

export const ModalProvider: FC<ModalProviderProps> = ({ children }) => {
  const [modals, setModals] = useState<Record<string, ModalState>>({});
  const [activeId, setActiveId] = useState<string | null>(null);
  const zIndexes = useRef<Record<string, number>>({});

  const addModal = (modal: ModalState) => {
    setModals({ ...modals, [modal.id]: modal });
  };

  const removeModal = (id: string) => {
    setModals((prev) => {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const { [id]: _, ...rest } = prev;

      return rest;
    });
  };

  const setActive = (id: string | null) => {
    setActiveId(id);

    if (typeof id === 'string') {
      setModals((prev) => {
        const rest = { ...prev };

        rest[id] = { ...rest[id], lastInteraction: Date.now() };

        return rest;
      });
    }
  };

  const context: ModalWindowContext = {
    modals,
    zIndexes,
    activeId,
    addModal,
    removeModal,
    setActive,
  };

  useEffect(() => {
    const sortedModals = Object.values(modals).toSorted(
      (a, b) => a.lastInteraction - b.lastInteraction,
    );

    sortedModals.forEach((modal) => {
      zIndexes.current[modal.id] = sortedModals.indexOf(modal) + 1;
    });
  }, [modals]);

  return (
    <ModalContext.Provider value={context}>{children}</ModalContext.Provider>
  );
};
