import { createContext } from 'react';

import type { WindowInstance } from '../types';

export const WindowContext = createContext<WindowInstance | null>(null);
