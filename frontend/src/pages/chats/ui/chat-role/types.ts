import type { ChatIdentity } from '@pages/chats/model';

export type ChatRoleProps = Partial<Pick<ChatIdentity, 'kind'>>;
