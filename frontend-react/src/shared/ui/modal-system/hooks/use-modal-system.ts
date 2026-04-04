import { createContext, useContext } from 'react';

import type { ModalWindowContext } from './types';

export const ModalContext = createContext<ModalWindowContext | null>(null);

export const useModalSystem = () => {
  const context = useContext(ModalContext);

  if (!context) {
    throw new Error('useModalSystem must be used within a ModalProvider');
  }

  return context;
};
