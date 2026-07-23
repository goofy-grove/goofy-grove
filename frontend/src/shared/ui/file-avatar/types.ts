import type { AvatarProps } from '@shared/ui/avatar';

export type FileAvatarProps = Omit<AvatarProps, 'src'> & {
  fileUid?: string | null;
  previewUrl?: string | null;
};
