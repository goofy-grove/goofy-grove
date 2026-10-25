import { createFileRoute } from '@tanstack/react-router';

import { ChatViewPage } from '@pages/chats';

export const Route = createFileRoute('/_main-layout/chats/$uid')({
  component: function ChatViewRoute() {
    const { uid } = Route.useParams();

    return <ChatViewPage uid={uid} />;
  },
});
