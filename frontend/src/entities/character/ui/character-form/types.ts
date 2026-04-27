export type CharacterFormProps = {
  name: string;
  description: string;
  isPending: boolean;
  submitLabel: string;

  onNameChange: (value: string) => void;
  onDescriptionChange: (value: string) => void;
  onSubmit: () => void;
};
