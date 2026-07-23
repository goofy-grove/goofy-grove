import { createFileRoute } from '@tanstack/react-router';

import { PersonaFormPage } from '@pages/personas';

export const Route = createFileRoute('/_main-layout/personas/$uid')({
  component: function PersonaEditPage() {
    const { uid } = Route.useParams();

    return <PersonaFormPage mode="edit" uid={uid} />;
  },
});
