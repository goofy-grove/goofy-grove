import { useCallback, useEffect, useRef, useState, type FC } from 'react';

import {
  ModalContext,
  type ModalState,
  type ModalWindowContext,
} from './hooks';

import type { ModalProviderProps } from './types';

export const ModalProvider: FC<ModalProviderProps> = ({ children }) => {
  const modals = useRef<Record<string, ModalState>>({});
  const [activeId, setActiveId] = useState<string | null>(null);
  const zIndexes = useRef<Record<string, number>>({});

  const addModal = useCallback((modal: ModalState) => {
    modals.current = { ...modals.current, [modal.id]: modal };
  }, []);

  const removeModal = useCallback((id: string) => {
    delete zIndexes.current[id];
    delete modals.current[id];
  }, []);

  const setActive = useCallback((id: string | null) => {
    setActiveId(id);

    if (typeof id === 'string') {
      modals.current = {
        ...modals.current,
        [id]: { ...modals.current[id], lastInteraction: Date.now() },
      };
    }
  }, []);

  const context: ModalWindowContext = {
    modals,
    zIndexes,
    activeId,
    addModal,
    removeModal,
    setActive,
  };

  useEffect(() => {
    const sortedModals = Object.values(modals.current).toSorted(
      (a, b) => a.lastInteraction - b.lastInteraction,
    );

    sortedModals.forEach((modal) => {
      zIndexes.current[modal.id] = sortedModals.indexOf(modal) + 1;
    });
  }, []);

  return (
    <ModalContext.Provider value={context}>{children}</ModalContext.Provider>
  );
};
