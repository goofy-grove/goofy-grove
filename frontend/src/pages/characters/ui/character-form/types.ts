export type CharacterFormValues = {
  name: string;
  description: string;
  avatarFile?: File;
};

export type CharacterFormProps = {
  initialValues: Pick<CharacterFormValues, 'name' | 'description'>;
  isPending: boolean;
  submitLabel: string;
  errorMessage?: string;
  avatarUid?: string;
  avatarPreviewUrl?: string;

  onCancel: () => void;
  onSubmit: (values: CharacterFormValues) => void;
};
