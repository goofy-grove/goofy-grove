import { createFileRoute } from '@tanstack/react-router';

import { ChatFormPage } from '@pages/chats';

export const Route = createFileRoute('/_main-layout/chats/new')({
  component: ChatFormPage,
});
