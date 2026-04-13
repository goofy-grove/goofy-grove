import { createFileRoute, isRedirect, redirect } from '@tanstack/react-router';

import { LoginPage } from '@pages/login';

export const Route = createFileRoute('/login')({
  component: LoginPage,
  beforeLoad: async ({ context }) => {
    if (context.auth.user) {
      throw redirect({
        to: '/',
        replace: true,
      });
    }

    try {
      await context.auth.getMe();

      throw redirect({
        to: '/',
        replace: true,
      });
    } catch (err) {
      if (isRedirect(err)) {
        throw err;
      }

      return;
    }
  },
});
