import type { PropsWithChildren } from 'react';

export type FormProps = PropsWithChildren<{
  isPending?: boolean;
  hasPortrait?: boolean;

  errorMessage?: string;
  submitLabel?: string;
  headingText?: string;
  headingHint?: string;

  avatarPreviewUrl?: string;
  avatarUid?: string;

  onSubmit?: () => void;
  onCancel?: () => void;
  onPortraitChange?: (file: File) => void;
}>;
