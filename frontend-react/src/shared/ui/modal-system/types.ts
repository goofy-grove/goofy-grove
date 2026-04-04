import type { PropsWithChildren } from 'react';

export type ModalProviderProps = PropsWithChildren<{
  maxModals: number;
  container?: string;
}>;
