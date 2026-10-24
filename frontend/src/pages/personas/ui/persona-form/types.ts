export type PersonaFormValues = {
  name: string;
  description: string;
  avatarFile?: File;
};

export type PersonaFormProps = {
  initialValues: Pick<PersonaFormValues, 'name' | 'description'>;
  isPending: boolean;
  submitLabel: string;
  errorMessage?: string;
  avatarUid?: string;

  onCancel: () => void;
  onSubmit: (values: PersonaFormValues) => void;
};
