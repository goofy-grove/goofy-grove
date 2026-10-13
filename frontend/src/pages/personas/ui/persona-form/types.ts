export type PersonaFormProps = {
  name: string;
  description: string;
  isPending: boolean;
  submitLabel: string;
  errorMessage?: string;
  avatarUid?: string;
  avatarPreviewUrl?: string;

  onNameChange: (value: string) => void;
  onDescriptionChange: (value: string) => void;
  onAvatarChange: (file?: File) => void;
  onCancel: () => void;
  onSubmit: () => void;
};
