import { createFileRoute, redirect } from '@tanstack/react-router';

import { MainPage } from '../../../../../pages/main';

export const Route = createFileRoute('/home')({
  component: MainPage,
  beforeLoad: async ({ context, matches, location }) => {
    const route = matches.at(-1);

    if (!route?.staticData?.isAuthRequired) {
      return;
    }

    const redirectToLogin = () => {
      throw redirect({
        to: '/login',
        replace: true,
        search: { redirect: location.href },
      });
    };

    try {
      const user = await context.auth.getMe();

      return { auth: { ...context.auth, user, isAuthenticated: true } };
    } catch {
      redirectToLogin();
    }

    if (route.staticData.isAuthRequired && !context.auth.isAuthenticated) {
      redirectToLogin();
    }
  },
  staticData: {
    isAuthRequired: true,
  },
});
