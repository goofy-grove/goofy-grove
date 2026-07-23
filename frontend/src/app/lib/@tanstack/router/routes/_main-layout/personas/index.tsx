import { createFileRoute } from '@tanstack/react-router';

import { PersonasPage } from '@pages/personas';

export const Route = createFileRoute('/_main-layout/personas/')({
  component: PersonasPage,
});
