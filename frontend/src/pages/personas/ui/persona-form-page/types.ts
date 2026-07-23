export type PersonaFormPageProps = {
  mode: 'create' | 'edit';
  uid?: string;
};

export type PersonaFormStateProps = {
  mode: 'create' | 'edit';
  uid?: string;
  initialName: string;
  initialDescription: string;
  avatarUid?: string | null;
};
