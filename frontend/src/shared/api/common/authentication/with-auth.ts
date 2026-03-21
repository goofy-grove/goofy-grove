import { refresh } from '../../auth/refresh';
import { api } from '../../axios';
import type { ApiFunction } from '../types';
import { authState } from './auth-state';

const MILLISECONDS_IN_SECOND = 1_000;

export const withAuth =
  <
    ApiFn extends ApiFunction<Args>,
    // NOTE: For better type inference of the result function arguments
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    Args = any,
  >(
    fn: ApiFn,
  ) =>
  async (...args: Parameters<ApiFn>): Promise<ReturnType<ApiFn>> => {
    try {
      await refresh();
    } catch {
      api.defaults.headers.common.Authorization = '';

      throw new Error('Not authorized');
    }

    if (!authState.token) {
      api.defaults.headers.common.Authorization = '';

      throw new Error('Not authorized');
    }

    if (Date.now() > authState.exp * MILLISECONDS_IN_SECOND) {
      // TODO: refresh tokens
      api.defaults.headers.common.Authorization = '';

      throw new Error('Token expired');
    }

    api.defaults.headers.common.Authorization = `Bearer ${authState.token}`;

    return fn(...args) as ReturnType<ApiFn>;
  };
