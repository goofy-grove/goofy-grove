import { createFileRoute } from '@tanstack/react-router';

import { CharactersPage } from '@pages/characters';

export const Route = createFileRoute('/_main-layout/characters/')({
  component: CharactersPage,
});
