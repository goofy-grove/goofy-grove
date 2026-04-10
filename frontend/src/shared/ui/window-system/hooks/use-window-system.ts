import { useContext } from 'react';

import { WindowContext } from '../model';

export const useWindowSystem = () => {
  const context = useContext(WindowContext);

  if (!context) {
    throw new Error('useWindowSystem must be used within a WindowProvider');
  }

  return context;
};
