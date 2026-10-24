import { createFileRoute } from '@tanstack/react-router';

import { ChatFormPage } from '@pages/chats';

export const Route = createFileRoute('/_main-layout/chats/$uid')({
  component: function ChatEditRoute() {
    const { uid } = Route.useParams();

    return <ChatFormPage mode="edit" uid={uid} />;
  },
});
