import compose from 'compose-function';

import { withQuery } from './with-query';
import { withRouter } from './with-router';
import { withStrictMode } from './with-strict-mode';

export const withProviders = compose(withQuery, withStrictMode, withRouter);
