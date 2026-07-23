import { type FC } from 'react';

import { Avatar } from '@shared/ui/avatar';
import { useFileUrl } from '@shared/ui/hooks';

import type { FileAvatarProps } from './types';

export const FileAvatar: FC<FileAvatarProps> = ({
  fileUid,
  previewUrl,
  ...props
}) => {
  const remoteUrl = useFileUrl(previewUrl ? null : fileUid);
  const src = previewUrl ?? remoteUrl ?? undefined;

  return <Avatar {...props} src={src} />;
};
