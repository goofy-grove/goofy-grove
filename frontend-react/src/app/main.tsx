import { createRoot } from 'react-dom/client';

import './assets/general.scss';

import { withProviders } from './providers';
import { initI18n } from './lib';

const rootElement = document.getElementById('root');

if (rootElement && !rootElement?.innerHTML) {
  const root = createRoot(rootElement);

  void initI18n().then(() => root.render(withProviders(() => null)()));
}
