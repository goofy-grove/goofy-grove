import { createRoot } from 'react-dom/client';

import './assets/general.scss';

import { initI18n } from '@shared/lib';

import { App } from './App';

const rootElement = document.getElementById('root');

if (rootElement && !rootElement?.innerHTML) {
  const root = createRoot(rootElement);

  void initI18n().then(() => root.render(<App />));
}
