import compose from 'compose-function';

import { withQuery } from './with-query';
import { withRouter } from './with-router';
import { withStrictMode } from './with-strict-mode';
import { withModal } from './with-modal';

export const withProviders = compose(
  withModal,
  withQuery,
  withStrictMode,
  withRouter,
);
