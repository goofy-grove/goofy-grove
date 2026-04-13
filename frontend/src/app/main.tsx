import { createRoot } from 'react-dom/client';

import './assets/general.scss';

import {
  PERSONA_CREATE_WINDOW_KEY,
  PERSONA_LIST_WINDOW_KEY,
  PersonaCreateWindow,
  PersonaListWindow,
} from '@entities/persona';

import { initI18n } from '@shared/lib';
import { registerWindow } from '@shared/ui';

import { App } from './App';

const rootElement = document.getElementById('root');

if (rootElement && !rootElement?.innerHTML) {
  const root = createRoot(rootElement);

  void initI18n().then(() => root.render(<App />));
}

registerWindow(PERSONA_LIST_WINDOW_KEY, PersonaListWindow);
registerWindow(PERSONA_CREATE_WINDOW_KEY, PersonaCreateWindow);
