import { createRoot } from 'react-dom/client';

import './assets/general.scss';

import { initI18n } from '../shared/lib';

import { withProviders } from './providers';

const rootElement = document.getElementById('root');

if (rootElement && !rootElement?.innerHTML) {
  const root = createRoot(rootElement);

  void initI18n().then(() => root.render(withProviders(() => null)()));
}
