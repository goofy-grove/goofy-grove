import { createFileRoute } from '@tanstack/react-router';

import { PersonaFormPage } from '@pages/personas';

export const Route = createFileRoute('/_main-layout/personas/new')({
  component: () => <PersonaFormPage mode="create" />,
});
