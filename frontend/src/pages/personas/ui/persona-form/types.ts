export type PersonaFormProps = {
  name: string;
  description: string;
  isPending: boolean;
  submitLabel: string;
  errorMessage?: string | null;
  avatarUid?: string | null;
  avatarPreviewUrl?: string | null;

  onNameChange: (value: string) => void;
  onDescriptionChange: (value: string) => void;
  onAvatarChange: (file: File | null) => void;
  onSubmit: () => void;
};
