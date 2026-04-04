import { ModalProvider } from '../../shared/ui';

import type { ReactNode } from 'react';

export const withModal = (component: () => ReactNode) => () => {
  return <ModalProvider maxModals={10}>{component()}</ModalProvider>;
};
