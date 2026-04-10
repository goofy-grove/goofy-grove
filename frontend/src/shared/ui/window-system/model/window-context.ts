import { createContext } from 'react';

import type { WindowContextType } from './types';

export const WindowContext = createContext<WindowContextType | null>(null);
