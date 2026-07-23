import { createFileRoute, redirect } from '@tanstack/react-router';

import { MainLayout } from '@pages/main-layout';

export const Route = createFileRoute('/_main-layout')({
  component: MainLayout,
  beforeLoad: async ({ context, location }) => {
    try {
      if (!context.auth.user) {
        await context.auth.getMe();
      }
    } catch {
      throw redirect({
        to: '/login',
        replace: true,
        search: { redirect: location.href },
      });
    }
  },
  staticData: {
    isAuthRequired: true,
  },
});
