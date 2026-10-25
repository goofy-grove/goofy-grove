import { toChatMessage, useMessagesQuery } from '@pages/chats/model';
import { ChatView } from '@pages/chats/ui/chat-view';

import { useAuth } from '@entities/auth';

import { useFileUrl } from '@shared/ui';

import type { ChatViewPageProps } from './types';

export const ChatViewPage = ({ uid }: ChatViewPageProps) => {
  const { user } = useAuth();
  const fileUrl =
    useFileUrl('file-01a0c57a-25eb-7370-b133-d7bae4105e5e') || undefined;

  const { data, fetchNextPage, hasNextPage } = useMessagesQuery(uid);

  const handleLoadOlder = async () => {
    await fetchNextPage();
  };

  const messages = data?.pages
    .map((page) =>
      page.messages.map((message) =>
        toChatMessage(message, {
          currentUserUid: '123',
          locale: 'en',
          ownerNames: {},
        }),
      ),
    )
    .flat();

  return (
    <div
      className="chat-view-page"
      style={{
        backgroundImage: `url(${fileUrl})`,
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'cover',
        height: '100%',
        display: 'flex',
        maxHeight: '100%',
        overflow: 'hidden',
      }}
    >
      <ChatView
        avatarUid={'file-01a0c57a-25eb-7370-b133-d7bae4105e5e'}
        title="Mega chat"
        subtitle="asdkfj"
        users={[]}
        currentUserId={user!.uid}
        characters={[]}
        myPersonas={[]}
        selectedPersonaId=""
        messages={messages || []}
        onPersonaChange={() => {}}
        onSend={() => {}}
        onTriggerCharacter={() => {}}
        onEditCharacter={() => {}}
        backgroundUrl={fileUrl}
        hasOlder={hasNextPage}
        loadOlder={handleLoadOlder}
      />
    </div>
  );
};
