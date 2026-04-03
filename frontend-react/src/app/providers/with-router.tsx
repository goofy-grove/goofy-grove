import { RouterProvider } from '@tanstack/react-router';

import { router } from '../lib';

export const withRouter =
  (component: () => React.ReactNode) => (): React.ReactNode => (
    <>
      <RouterProvider router={router} />

      {component()}
    </>
  );
