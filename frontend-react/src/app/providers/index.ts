import compose from 'compose-function';

import { withLocaleSwitcher } from './with-locale-switcher';
import { withQuery } from './with-query';
import { withRouter } from './with-router';
import { withStrictMode } from './with-strict-mode';

export const withProviders = compose(
  withLocaleSwitcher,
  withQuery,
  withStrictMode,
  withRouter,
);
