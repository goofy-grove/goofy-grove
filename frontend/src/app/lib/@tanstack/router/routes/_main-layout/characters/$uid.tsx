import { createFileRoute } from '@tanstack/react-router';

import { CharacterFormPage } from '@pages/characters';

export const Route = createFileRoute('/_main-layout/characters/$uid')({
  component: function CharacterEditPage() {
    const { uid } = Route.useParams();

    return <CharacterFormPage mode="edit" uid={uid} />;
  },
});
