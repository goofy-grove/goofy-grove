import { FileAvatar } from '@shared/ui';

import type { IdentityAvatarProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const IdentityAvatar: FC<IdentityAvatarProps> = ({
  identity,
  small = false,
}) => (
  <span
    className={`chat-avatar chat-avatar--${identity.kind} ${small ? 'chat-avatar--small' : ''}`}
    aria-hidden="true"
  >
    <FileAvatar
      fileUid={identity.avatarUid}
      previewUrl={identity.previewUrl}
      fallback={identity.initials}
      alt=""
    />
  </span>
);
