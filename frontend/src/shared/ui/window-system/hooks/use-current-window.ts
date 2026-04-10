import { useContext } from 'react';

import { WindowContext } from '../services';

import { useWindow } from './use-window';

export const useCurrentWindow = () => {
  const windowState = useContext(WindowContext);

  if (!windowState) {
    throw new Error('useCurrentWindow must be used within a WindowProvider');
  }

  const windowMethods = useWindow(windowState.type, windowState.instanceId);

  return { ...windowState, ...windowMethods };
};
