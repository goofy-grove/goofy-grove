import { createRoot } from 'react-dom/client';

import './assets/general.scss';

import {
  CHARACTER_FORM_WINDOW_KEY,
  CHARACTER_LIST_WINDOW_KEY,
  CharacterFormWindow,
  CharacterListWindow,
} from '@entities/character';
import {
  PERSONA_FORM_WINDOW_KEY,
  PERSONA_LIST_WINDOW_KEY,
  PersonaFormWindow,
  PersonaListWindow,
} from '@entities/persona';

import { initI18n } from '@shared/lib';
import { registerWindow } from '@shared/ui';

import { App } from './App';

const rootElement = document.getElementById('root');

if (rootElement && !rootElement?.innerHTML) {
  const root = createRoot(rootElement);

  registerWindow(PERSONA_LIST_WINDOW_KEY, PersonaListWindow);
  registerWindow(PERSONA_FORM_WINDOW_KEY, PersonaFormWindow);
  registerWindow(CHARACTER_LIST_WINDOW_KEY, CharacterListWindow);
  registerWindow(CHARACTER_FORM_WINDOW_KEY, CharacterFormWindow);

  void initI18n().then(() => root.render(<App />));
}
