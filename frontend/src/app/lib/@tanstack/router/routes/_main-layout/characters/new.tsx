import { createFileRoute } from '@tanstack/react-router';

import { CharacterFormPage } from '@pages/characters';

export const Route = createFileRoute('/_main-layout/characters/new')({
  component: () => <CharacterFormPage mode="create" />,
});
