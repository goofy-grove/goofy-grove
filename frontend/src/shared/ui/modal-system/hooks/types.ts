import type { RefObject } from 'react';

export type ModalState = {
  id: string;
  isOpen: boolean;
  lastInteraction: number;
};

export type ModalRegistry = Record<string, ModalState>;
export type ModalZIndexRegistry = Record<string, number>;

export type ModalWindowContext = {
  modals: RefObject<ModalRegistry>;
  zIndexes: RefObject<ModalZIndexRegistry>;
  activeId: string | null;

  addModal: (modal: ModalState) => void;
  removeModal: (id: string) => void;

  setActive: (id: string | null) => void;
};
