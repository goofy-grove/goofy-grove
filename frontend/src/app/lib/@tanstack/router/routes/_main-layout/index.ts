import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/_main-layout/')({
  beforeLoad: () => {
    throw redirect({ to: '/personas', replace: true });
  },
});
