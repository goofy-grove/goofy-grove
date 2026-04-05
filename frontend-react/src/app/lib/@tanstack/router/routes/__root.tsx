import { createRootRouteWithContext, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';

import type { RouterContext } from '../types';

export const Route = createRootRouteWithContext<RouterContext>()({
  component: () => (
    <>
      <Outlet />

      {import.meta.env.DEV && <TanStackRouterDevtools />}
    </>
  ),
});
