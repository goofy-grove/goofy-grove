export type ChatFormValues = {
  name: string;
  avatarFile?: File;
};

export type ChatFormProps = {
  initialValues: Pick<ChatFormValues, 'name'>;
  isPending: boolean;
  submitLabel: string;
  errorMessage?: string;
  avatarUid?: string;

  onCancel: () => void;
  onSubmit: (values: ChatFormValues) => void;
};
