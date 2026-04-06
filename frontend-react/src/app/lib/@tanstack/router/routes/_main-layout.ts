import { createFileRoute } from '@tanstack/react-router';

import { MainLayout } from '../../../../../pages/main-layout';

export const Route = createFileRoute('/_main-layout')({
  component: MainLayout,
});
