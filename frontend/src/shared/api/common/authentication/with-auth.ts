import { refresh } from '@shared/api/auth/refresh';
import { api } from '@shared/api/axios';
import type { ApiFunction } from '@shared/api/common/types';

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
  async (...args: Parameters<ApiFn>): Promise<Awaited<ReturnType<ApiFn>>> => {
    if (
      !authState.token ||
      Date.now() > authState.exp * MILLISECONDS_IN_SECOND
    ) {
      try {
        await refresh();
      } catch {
        api.defaults.headers.common.Authorization = '';

        throw new Error('Not authorized');
      }
    }

    api.defaults.headers.common.Authorization = `Bearer ${authState.token}`;

    return fn(...args) as Awaited<ReturnType<ApiFn>>;
  };
