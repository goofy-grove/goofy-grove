import { createFileRoute, redirect } from '@tanstack/react-router';

import { MainPage } from '@pages/main';

export const Route = createFileRoute('/_main-layout/_home')({
  component: MainPage,
  beforeLoad: async ({ context, matches, location }) => {
    const route = matches.at(-1);

    try {
      if (route?.staticData?.isAuthRequired && !context.auth.user) {
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
